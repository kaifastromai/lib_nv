#![feature(proc_macro_diagnostic)]
use proc_macro::{Diagnostic, Level, TokenStream};
use proc_macro2 as pm2;
use quote::{format_ident, quote, ToTokens, __private::Span};
use std::{fs::File, hash::*, io::Read};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, DeriveInput, Field,
};
const SERDE_EXPORT_PATH: &str = "common::exports::serde";
use components_track::comp_link::COMPONENTS;

trait StringExt {
    fn to_snake_case(&self) -> String;
}
impl StringExt for str {
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        for (i, c) in self.chars().enumerate() {
            if i == self.len() - 1 {
                result.push_str(c.to_lowercase().to_string().as_str());

                break;
            }
            if !c.is_uppercase() && self.chars().nth(i + 1).unwrap().is_uppercase() {
                result.push_str(c.to_lowercase().to_string().as_str());
                result.push('_');
            } else {
                result.push_str(c.to_lowercase().to_string().as_str());
            }
        }
        result
    }
}
#[proc_macro_derive(Resource)]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    //it is an error of the type has generic parameters
    if !input.generics.params.is_empty() {
        Diagnostic::new(Level::Error, "Resource can not have generic parameters").emit();
    }
    //call the structs new function
    let struct_impl = quote! {
        impl ResrcTy for #name{
            fn get_mut(&mut self)->&mut dyn Any{
               self
            }
        }
        impl ResrcTy for &'static #name{
            fn get_mut(&mut self)->&mut  dyn Any{
                self
            }
        }
    };

    struct_impl.into()
}
#[proc_macro_derive(Param)]
pub fn param_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    //it is an error of the type has generic parameters
    if !input.generics.params.is_empty() {
        Diagnostic::new(Level::Error, "Param can not have generic parameters").emit();
    }
    //call the structs new function
    let struct_impl = quote! {
        impl ParamTy for #name{
            fn get_param(self)->Box<dyn Any>{
                Box::new(self)
            }
        }

    };

    struct_impl.into()
}

//Generates implementation of actionfn for any function
#[proc_macro_attribute]
pub fn undo_action(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemFn);
    let attr = parse_macro_input!(attr as syn::AttributeArgs);

    //get function arguments
    let mut args = item.sig.inputs.iter();

    //if there are more than 1 argument, it is an error
    if args.len() > 1 {
        Diagnostic::new(Level::Error, "Undo action can only have one argument").emit();
    }
    //get first argument

    let resrc_arg = args.next();

    //get the inner T in the Resrc<T>
    let inner_t = match resrc_arg {
        Some(syn::FnArg::Typed(syn::PatType { ref ty, .. })) => match ty.as_ref() {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let s = path.segments.last();
                match s {
                    Some(seg) => {
                        let arg = &seg.arguments;
                        let arg = match arg {
                            syn::PathArguments::None => {
                                Diagnostic::new(Level::Error, "Unrecognized resource argument. The resource must state the type it needs").emit();
                                panic!()
                            }
                            syn::PathArguments::AngleBracketed(a) => a.args.iter().next().unwrap(),
                            syn::PathArguments::Parenthesized(_) => {
                                Diagnostic::new(Level::Error, "Unrecognized resource argument. The resource must state the type it needs").emit();
                                panic!()
                            }
                        };

                        quote! {#arg}
                    }
                    None => {
                        Diagnostic::new(Level::Error, "Unrecognized resource argument").emit();
                        panic!()
                    }
                }
            }
            _ => {
                Diagnostic::new(Level::Error, "Unrecognized resource argument").emit();
                panic!()
            }
        },

        _ => quote! {()},
    };

    //get the name of the resrc argument
    let resrc_arg_name = match resrc_arg {
        Some(syn::FnArg::Typed(syn::PatType { ref pat, .. })) => match pat.as_ref() {
            syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => ident.to_string(),
            _ => {
                Diagnostic::new(Level::Error, "Undo action can only have one argument").emit();
                panic!()
            }
        },
        _ => String::from("resrc"),
    };
    let resrc_arg_name = syn::Ident::new(&resrc_arg_name, Span::call_site());

    let resrc_arg = match resrc_arg {
        Some(resrc_arg) => resrc_arg.clone(),
        //This action does not need any resource. Construct an empty resource
        None => {
            //construct a new fnarg
            let fn_arg: syn::FnArg = syn::parse_str("resrc:Resrc<()>").unwrap();
            fn_arg
        }
    };
    //get the function name
    let name = &item.sig.ident;

    //get the function return type
    let _ret_type = &item.sig.output;
    //get the function body
    let body = &item.block;
    let new_decl = quote! {
        pub fn #name(mir:&mut Mir,mut #resrc_arg_name: Resrc<&dyn ResrcTy>) -> Result<()> {
            let #resrc_arg_name :&#inner_t= #resrc_arg_name.0.get_resource().downcast_ref::<#inner_t>().unwrap();
            #body
        }
    };
    new_decl.into()
}
#[proc_macro_derive(Action)]
pub fn action_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let struct_impl = quote! {
        impl<'a,R:ResrcTy,P:ParamTy> ActionTy for #name<'a,R,P>{
            fn exec(&mut self, mir:&mut Mir)->Result<Box<dyn ResrcTy>>{
                self.exec(mir)
            }
            fn undo(&mut self,mir:&mut Mir,rsrc:Resrc<&dyn ResrcTy>)->Result<()>{
                let resrc=rsrc.0.get_resource().downcast_ref::<R>().unwrap();
                self.undo(mir,rsrc)
            }
        }
    };

    struct_impl.into()
}

//Takes a comma seperated list of Component structs with predefined fields and generates a vector of EComponentGraphType
#[proc_macro]
pub fn arch_sig(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ExprArray);
    let mut enum_exprs: Vec<pm2::TokenStream> = Vec::new();

    //iterate through all struct expressions
    for expr in item.elems {
        if let syn::Expr::Struct(s) = expr {
            let expr_name = s.path.segments.last().unwrap();
            let enum_expr = quote! {
               EComponentGraphTypes::#expr_name(#s)
            };
            enum_exprs.push(enum_expr);
        }
    }
    quote! {
        vec![#(#enum_exprs),*]
    }
    .into()
}

///Procedural macro that generates an enum of ComponentTypes by looping through the AST for
/// all instances of the ComponentType attribute
#[proc_macro_attribute]
pub fn generate_component_types(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //iterate over all items in the stream
    let items = parse_macro_input!(item as syn::ItemMod);
    let mut comp_type_idents: Vec<syn::Ident> = Vec::new();
    let content = items.content.as_ref().unwrap();

    for item in content.1.iter() {
        if let syn::Item::Struct(s) = item {
            //get attributes
            let mut attrs = s.attrs.iter();
            //check if one of the attribs is "component"
            if attrs.any(|attr| attr.path.is_ident("component")) {
                //get the name of the struct
                let name = &s.ident;
                //add the name to the list of component types
                comp_type_idents.push(name.clone());
            }
        }
    }
    let comp_types_enum = quote! {
        use crate::ecs::component::components::*;
        //This is simply an enum that lists all the component types
        #[derive(Debug,Clone)]
        #[nvproc::serde_derive]
        pub enum EComponentTypes{
            #(#comp_type_idents,)*
        }
        //This is an enum that lists and owns all the component types
        #[nvproc::serde_derive]

        pub enum EComponentGraphTypes{
            #(#comp_type_idents(#comp_type_idents),)*
        }
        impl EComponentGraphTypes{
           ///This inserts the component this enum owns into the storage.
            pub fn insert_component_into_storage(self, storage:&mut crate::ecs::Storage, owning_entity:crate::ecs::Id){
                match self{
                    #(Self::#comp_type_idents(comp_type)=>{
                        storage.insert_component(owning_entity,comp_type);
                    },)*
                }

            }
        }
       impl TypeIdTy for EComponentGraphTypes{
            fn get_type_id_ref(&self)->TypeId{
                match self{
                    #(Self::#comp_type_idents(t)=>t.get_type_id_ref(),)*
                }
            }
        }
    };

    //  let mut comp_type_strings = comp_type_strings.into_iter();
    quote! {
       #comp_types_enum
        #items
    }
    .into()
}

///Computes a 64bit type_id based on the hash of the name of the type
#[proc_macro_derive(TypeId)]
pub fn type_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    //get hash of the component name
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.to_string().hash(&mut hasher);
    let hash_id = hasher.finish();
    let struct_impl = quote! {

        impl TypeIdTy for #name{
            fn get_type_id()->TypeId{
                TypeId::new(#hash_id)
            }
            fn get_type_id_ref(&self)->TypeId{
                TypeId::new(#hash_id)
            }
            fn get_name()->&'static str{
                stringify!(#name)
            }
            fn get_name_ref(&self)->&'static str{
                stringify!(#name)
            }
        }
        impl crate::ecs::ComponentTypeIdTy for #name{}
    };

    struct_impl.into()
}

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_str = name.to_string();
    let generics = &input.generics;
    //remove default params for the generics
    let mut impl_generics = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(t) => syn::GenericParam::Type(syn::TypeParam {
                default: None,
                ..t.clone()
            })
            .to_token_stream(),
            syn::GenericParam::Const(c) => syn::GenericParam::Const(syn::ConstParam {
                default: None,
                ..c.clone()
            })
            .to_token_stream(),
            syn::GenericParam::Lifetime(_) => todo!(),
        })
        .collect::<Vec<_>>();

    let generic_type_names = generics.params.iter().map(|p| match p {
        syn::GenericParam::Type(syn::TypeParam { ident, .. }) => ident.to_token_stream(),
        syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. }) => {
            quote! {#lifetime}
        }
        syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote! {#ident},
    });

    //check if component has generic parameters
    if generics.params.is_empty() {
        impl_generics = vec![quote! {}];
    }
    let impl_block = quote! {
        impl <#(#impl_generics),*> crate::ecs::ComponentTy for #name <# (#generic_type_names),*>{
          fn clean(&mut self){
             todo!()
          }
          fn get_component_name(&self)->&'static str{
             #name_str
          }
          fn get_any(&self)->&dyn std::any::Any{
             self
          }
          fn get_any_mut(&mut self)->&mut dyn std::any::Any{
             self
          }
        }


    };
    impl_block.into()
}
///Decorates the item with the necessary derives and such for the component
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;
    let name_ident_caps = name.to_string().to_uppercase();
    //convert to ident
    let name_ident = syn::Ident::new(&name_ident_caps, name.span());
    let name_str = name.to_string();

    input.attrs.append(
        &mut syn::Attribute::parse_outer
            .parse2(quote! {
                #[derive(Component, Default,nvproc::TypeId)]
                #[repr(C)]
                #[nvproc::serde_derive]

            })
            .unwrap(),
    );
    quote! {
        #[distributed_slice(COMPONENTS)]
        pub static #name_ident: &'static str =#name_str;
        #input
    }
    .into()
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

            pub fn get_mut<'a, T:crate::ecs::Component>(&'a mut self)->&'a mut HashMap<IndexType,T>{
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
            pub fn set_mark_for_deletion(&mut self, entity:IndexType, is_deleted:bool){
                #(

                    match self.#sn.get_mut(&entity){
                        Some(c)=>{
                            c.set_is_deleted(is_deleted);
                        },
                        None=>()

                    };


            )*
            }
            //Returns a new Components object with all the components associated with the given entity
            pub fn get_components(&self, entity_id:IndexType)->Components{
                let mut c:Components = Default::default();

                #(
                  match self.#sn.get(&entity_id){
                    Some(comp)=>   {c.#sn.insert(entity_id,comp.clone());},
                    None=>{}
                  }
                )*
                c

            }
            pub fn get_components_ref<'a>(&'a self, entity_id:IndexType)->Result<Vec< ComponentRef<'a>>, &'static str>{
                let mut c:Vec<ComponentRef<'a>> = Vec::new();
                #(
                   match self.#sn.get(&entity_id){
                       Some(cl)=>{
                    c.push(ComponentRef::<'a>::#n(cl));}
                None=>()
                };
                )*
                if(c.is_empty()){
                    return Err("No components found for the given entity")
                }
                Ok(c)
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
        #[derive(Default, Clone)]
        pub struct Components {
            #(pub #sn:HashMap<IndexType,components:: #n>,)*
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

//Adds serde's Serialize and Deserialize derive macros to the given struct,
//and optionally accept an additional parameter to specify the crate name
#[proc_macro_attribute]
pub fn serde_derive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::Item);
    let derive_attr = quote! {
        #[derive(Serialize,Deserialize)]
        #[serde(crate = #SERDE_EXPORT_PATH)]
    };

    match input {
        syn::Item::Struct(ref mut s) => {
            s.attrs.append(
                &mut syn::Attribute::parse_outer
                    .parse2(quote! {
                       #derive_attr
                    })
                    .unwrap(),
            );
        }
        syn::Item::Enum(ref mut e) => {
            e.attrs.append(
                &mut syn::Attribute::parse_outer
                    .parse2(quote! {
                       #derive_attr
                    })
                    .unwrap(),
            );
        }
        _ => {}
    }

    input.into_token_stream().into()
}
#[proc_macro]
pub fn type_name(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::Type);
    let name = input.to_token_stream().to_string();
    quote! {
        #name
    }
    .into()
}
#[proc_macro]
pub fn build_archetype_descriptor(item: TokenStream) -> TokenStream {
    todo!()
}
///Takes a path to a file, and outputs result as static string
#[proc_macro]
pub fn file_to_static_string(file: TokenStream) -> TokenStream {
    //interpret item as a string literal
    let input = syn::parse_macro_input!(file as syn::LitStr);
    let path = input.value();
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let contents = syn::LitStr::new(&contents, Span::call_site());
    quote! {
        #contents
    }
    .into()
}

#[cfg(test)]
mod tests;
