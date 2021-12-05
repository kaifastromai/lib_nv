#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, DeriveInput, Field,
};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    //list of private field idents
    let private_fields = ["owning_entity", "_is_marked_del"];

    //get all fields of struct
    let fields: Vec<Field> = if let syn::Data::Struct(data) = input.data {
        //return iterator that ignores private fields
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
    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());

    let prop_name = format_ident!("{}_prop", name);
    let gen = quote! {
        struct #prop_name {
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
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    component_type:ComponentType
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
    let mut input = parse_macro_input!(item as syn::ItemMod);
    let mut components_struct = syn::ItemStruct::parse
        .parse2(quote! {
            struct Components {
            }
        })
        .unwrap();
    let mut component_types = syn::ItemEnum::parse
        .parse2(quote! {
            enum ComponentType{

            }
        })
        .unwrap();
    let mut gen = quote! {};
    for content in input.content.iter_mut() {
        for item in content.1.iter_mut() {
            if let syn::Item::Struct(ref mut struct_item) = item {
                let name = struct_item.ident.clone();
                //convert name to camel case
                let camel_name = name.to_string();
                //split camel word on capital letters
                let mut split_name = camel_name.split(|c: char| c.is_uppercase());
                let mut component_name_string = String::new();
                //iterate and skip the first split
                for word in split_name.by_ref() {
                    component_name_string.push_str(word.to_lowercase().as_str());
                    component_name_string.push('_');
                }
                //create new Ident from string
                let component_name =
                    syn::Ident::new(component_name_string.trim_end_matches('_'), name.span());

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
                //add the component type to the components struct
                if let syn::Fields::Named(fields) = &mut components_struct.fields {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! {
                                #component_name:std::collections::HashMap<IndexType,#name>
                            })
                            .unwrap(),
                    );
                }
            }
        }

        gen.extend(quote! {
            #component_types
        });
    }
    //add to gen
    input.content.as_mut().unwrap().1.push(
        syn::Item::parse
            .parse2(quote! {
                #gen
            })
            .unwrap(),
    );
    input.content.as_mut().unwrap().1.push(
        syn::Item::parse
            .parse2(quote! {
                #components_struct
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
