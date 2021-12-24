#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::{format_ident, quote, ToTokens};
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
                #[derive(nvproc::Component, Clone)]
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
    let mut component_refs = syn::ItemEnum::parse
        .parse2(quote! {
            //An enum that holds references to a components
            pub enum ComponentRef<'a>{
            }
        })
        .unwrap();
    let mut gen = quote! {};
    for content in input.content.iter_mut() {
        for item in content.1.iter_mut() {
            if let syn::Item::Struct(ref mut struct_item) = item {
                let item = struct_item;

                let name = item.ident.clone();
                //convert name to snake case
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
                component_refs.variants.push(
                    syn::Variant::parse
                        .parse2(quote! {
                           #name (&'a components:: #name)
                        })
                        .unwrap(),
                );

                //add component attribute to struct
                item.attrs.append(
                    &mut syn::Attribute::parse_outer
                        .parse2(quote! {
                            #[nvproc::component]
                        })
                        .unwrap(),
                );
            }
        }

        gen.extend(quote! {
            #component_refs;

        });
        gen.extend(quote! {
            #component_types;

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
                  #(ecs::components::ComponentType::#n=>std::mem::transmute(&self.#sn),)*
                }
            };
            m
            }

            pub fn get_mut<T:crate::ecs::Component>(&mut self)->&mut HashMap<IndexType,T>{
                let m:&mut HashMap<IndexType,T> =unsafe{match T::get_type(){
                  #(ecs::components::ComponentType::#n=>std::mem::transmute(&mut self.#sn),)*
                }
            };
            m
            }
            pub fn merge(&mut self, other:Self){
                #(self.#sn.extend(other.#sn.into_iter());)*
            }
            pub fn delete_components(&mut self, entity:IndexType){
                #(self.#sn.remove(&entity);)*
            }
            //Returns a new Components object with all the components associated with the given entity
            pub fn get_components(&self, entity_id:IndexType)->Components{
                let mut c:Components = Default::default();

                #(
                   let cl= self.#sn.get(&entity_id).unwrap().clone();
                    c.#sn.insert(entity_id,cl);)*
                c

            }

        }
    };

    //add to gen
    input.content.as_mut().unwrap().1.push(
        syn::Item::parse
            .parse2(quote! {
                #component_types
            })
            .unwrap(),
    );
    let mut input_stream = input.into_token_stream();
    input_stream.extend(quote! {
        use components::*;
        #component_refs
    });

    let components_struct = quote! {
        #[derive(Default)]
        pub struct Components {
            #(pub #sn:HashMap<IndexType,#n>,)*
        }
    };
    //convert the list of names into a vector of ints from 0 to n
    let vec_i = n.iter().enumerate().map(|(i, _)| i as u32);
    let vec_size = vec_i.clone().count();
    let components_iterator = quote! {
        impl ecs::components::ComponentType{
            pub fn from_u32(i:u32)->ecs::components::ComponentType{
              let comp= match i{
                     #(#vec_i=>ecs::components::ComponentType::#n,)*
                     _=>panic!("Invalid component type")
               };
               comp
            }
                    ///Creates an iterator over all the types of all components
                    pub fn type_iter()->ComponentIter{
                        ComponentIter{current_index:0}
                    }
        }

        pub struct ComponentIter{current_index:usize}
        pub struct ComponentRefIter<'a>{current_index:usize,em:&'a mut EntityManager, entity:IndexType }

        impl Iterator for ComponentIter{
            type Item=ecs::components::ComponentType;

            fn next(&mut self)-> Option<Self::Item>{
                let res =match self.current_index>=#vec_size{
                    true=>
                    None,
                    false=>Some(ecs::components::ComponentType::from_u32(self.current_index as u32))

                };
                self.current_index+=1;
                res
            }
        }
        // impl<'a> Iterator for ComponentRefIter<'a>{
        //     type Item=ecs::ComponentRef<'a>;

        //     fn next(&mut self)->Option<Self::Item>{
        //         let res=match self.current_index>=#vec_size{
        //             true=>None,
        //             false=>{
        //                 let comp=ecs::components::ComponentType::from_u32(self.current_index as u32);
        //                 let comp_ref= match comp{
        //                    #(components::ComponentType:: #n=>{ecs::ComponentRef::<'a>::#n (self.em.get_component_mut::<components:: #n>(self.entity).unwrap())},)*
        //                 };
        //                 Some(comp_ref)
        //             }
        //         };
        //         self.current_index+=1;
        //         res

        //     }
        // }
    };

    input_stream.extend(quote! {
              #components_struct
              #impl_block
    });

    quote! {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused_imports)]
        #input_stream
        #components_iterator
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
