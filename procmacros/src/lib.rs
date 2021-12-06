#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, DeriveInput, Field,
};
use utils::StringExt;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    //list of private field idents
    let private_fields = ["owning_entity", "_is_marked_del"];

    //get all fields of struct
    let mut fields: Vec<Field> = if let syn::Data::Struct(data) = input.data {
        data.fields
            .into_iter()
            .filter(|f| match &f.ident {
                Some(i) => !private_fields.contains(&i.to_string().as_str()),
                _ => true,
            })
            .collect()
    } else {
        panic!("{} must be struct", name)
    };
    //make all fields public
    for field in &mut fields {
        field.vis = syn::Visibility::Public(syn::VisPublic {
            pub_token: Default::default(),
        });
    }
    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());

    let prop_name = format_ident!("{}Prop", name);
    let gen = quote! {
        pub struct #prop_name {
            #(#fields,)*

        }

        impl Component for #name {
           type Properties=#prop_name;
           fn new(owning_entity:IndexType, props:Self::Properties)->Self{
            Self{
                _is_marked_del:false,
                owning_entity,
                #(#field_names:props.#field_names,)*
            }
           }
            fn get_owning_entity(&self) -> IndexType {
                self.owning_entity
            }
            fn set_owning_entity(&mut self, entity:IndexType) {
                self.owning_entity = entity;
            }
            fn set_is_deleted(&mut self, is_deleted:bool){
                self._is_marked_del=is_deleted;
            }
            fn get_is_deleted(&self)->bool{
                self._is_marked_del
            }
            fn get_type()->components::ComponentType{
                components::ComponentType::#name
            }

        }
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.owning_entity == other.owning_entity
            }
        }
        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.owning_entity.hash(state);
            }
        }

    };
    gen.into()
}
///Generate a component struct for a given struct, implements Component trait and adds a field for the owning entity
/// It also generates PartialEq and Hash implementations
/// # Example
/// ```
/// #[component]
/// struct MyComponent {
///    field1: u32,
///   field2: u32,
/// }
/// ```
/// # Output
/// ```
/// #[derive(Component)]
/// struct MyComponent {
///   field1: u32,
///  field2: u32,
/// owning_entity: Option<IndexType>,
/// id: u128,
/// }
/// impl Component for MyComponent {
///   fn get_component_bits() -> u128 {
///    0x1
///  }
/// fn get_owning_entity(&self) -> Option<IndexType> {
///   self.owning_entity
/// }
/// fn set_owning_entity(&mut self, entity:Option<IndexType>) {
///  self.owning_entity = entity;
/// }
/// }
/// ```
///
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemStruct);
    input.attrs.append(
        &mut syn::Attribute::parse_outer
            .parse2(quote! {
                #[derive(nvproc::Component)]
            })
            .unwrap(),
    );
    if let syn::Fields::Named(fields) = &mut input.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    owning_entity:IndexType
                })
                .unwrap(),
        );
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    _is_marked_del:bool
                })
                .unwrap(),
        );

        quote! {
            #input
        }
        .into()
    } else {
        panic!("Only structs with named fields are supported");
    }
}
#[proc_macro_attribute]
///Generates a component type for every struct in the input module
pub fn gen_components(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut snake_names = Vec::<String>::new();
    let mut names = Vec::<syn::Ident>::new();
    let mut input = parse_macro_input!(item as syn::ItemMod);

    let mut component_types = syn::ItemEnum::parse
        .parse2(quote! {
            #[derive(Eq,PartialEq, Copy, Clone, Ord, PartialOrd)]
            pub enum ComponentType{


            }

        })
        .unwrap();
    let mut gen = quote! {};
    for content in input.content.iter_mut() {
        for item in content.1.iter_mut() {
            if let syn::Item::Struct(ref mut struct_item) = item {
                let name = struct_item.ident.clone();
                //convert name to snακε case
                let snake_name = name.to_string().as_str().to_snake_case();
                snake_names.push(snake_name);
                names.push(name.clone());
                //create new Ident from string

                Diagnostic::new(Level::Note, format!("Generating data for {}", name)).emit();
                //add the component type to the enum
                component_types.variants.push(
                    syn::Variant::parse
                        .parse2(quote! {
                            #name
                        })
                        .unwrap(),
                );

                //add component attribute to struct
                struct_item.attrs.append(
                    &mut syn::Attribute::parse_outer
                        .parse2(quote! {
                            #[nvproc::component]
                        })
                        .unwrap(),
                );
            }
        }

        gen.extend(quote! {
            #component_types
        });
    }

    //add impl block for Components that defines function get<T:Component>
    let names_iter = names.iter();

    let snake_iter: Vec<syn::Ident> = names_iter
        .clone()
        .map(|n| syn::Ident::new(n.to_string().as_str().to_snake_case().as_str(), n.span()))
        .collect();
    let sn = &snake_iter;
    let n = &names;
    Diagnostic::new(
        Level::Note,
        format!("Size of snake_names: {}", snake_names.len()),
    )
    .emit();
    let impl_block = quote! {
        impl Components{
            pub fn get<T:crate::ecs::Component>(&self)->&HashMap<IndexType,T>{

                let m:&HashMap<IndexType,T> =unsafe{match T::get_type(){
                  //#(ecs::components:ComponentType:#names_iter)* =>&self.fields
                  #(ecs::components::ComponentType::#n=>std::mem::transmute(&self.#sn),)*
                  //ecs::components::ComponentType::Fields=>&self.fields,
                }
            };
            m
            }
            pub fn get_mut<T:crate::ecs::Component>(&mut self)->&mut HashMap<IndexType,T>{
                let m:&mut HashMap<IndexType,T> =unsafe{match T::get_type(){
                  //#(ecs::components:ComponentType:#names_iter)* =>&self.fields
                  #(ecs::components::ComponentType::#n=>std::mem::transmute(&mut self.#sn),)*
                  //ecs::components::ComponentType::Fields=>&self.fields,
                }
            };
            m
            }
        }
    };

    //add to gen
    input.content.as_mut().unwrap().1.push(
        syn::Item::parse
            .parse2(quote! {
                #gen
            })
            .unwrap(),
    );

    let components_struct = quote! {
        #[derive(Default)]
        pub struct Components {
            #(#sn:HashMap<IndexType,#n>,)*
        }
    };

    input.content.as_mut().unwrap().1.push(
        syn::Item::parse
            .parse2(quote! {
                #components_struct
            })
            .unwrap(),
    );

    input.content.as_mut().unwrap().1.push(
        syn::Item::parse
            .parse2(quote! {
                #impl_block
            })
            .unwrap(),
    );

    quote! {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused_imports)]
        #input

    }
    .into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
