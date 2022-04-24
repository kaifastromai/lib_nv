pub mod archetypes;
pub mod relationship;

use common::components::*;
use common::exports::serde::*;
use common::exports::*;
use common::type_id::*;
use components_track::comp_link::COMPONENTS;
use linkme::distributed_slice;

use std::{
    default,
    path::{Path, PathBuf},
};

#[nvproc::generate_component_types]
pub mod components {
    use super::*;
    ///A general location component that will give basic tracking capabilities to the engine
    ///
    #[nvproc::bincode_derive]
    #[nvproc::serde_derive]
    pub struct GenericLocation {
        pub name: String,
        pub description: String,
    }
    #[nvproc::bincode_derive]
    #[nvproc::serde_derive]
    pub enum ELocation {
        Generic(GenericLocation),
    }

    impl Default for ELocation {
        fn default() -> Self {
            ELocation::Generic(GenericLocation {
                name: "".to_string(),
                description: "".to_string(),
            })
        }
    }
    #[component]
    pub struct LocationComponent {
        pub location: ELocation,
    }

    use nvproc::{component, Component};

    #[distributed_slice(COMPONENTS)]
    pub static ARCHETYPE: &'static str = "Archetype";
    pub struct ArchetypeComponent {
        pub archetype_name: String,
    }

    pub trait BinaryTy {
        fn to_bytes(&self) -> Vec<u8>;
    }

    #[distributed_slice(COMPONENTS)]
    pub static STRING_FIELD: &'static str = "StringField";

    #[component]
    pub struct StringFieldComponent {
        pub name: String,
        pub value: String,
    }

    #[distributed_slice(COMPONENTS)]
    pub static NUMERIC_FIELD: &'static str = "NumericField";
    #[component]
    pub struct NumericalFieldComponent {
        pub name: String,
        pub value: f32,
    }
    #[component]
    pub struct VideoComponent {
        video_name: String,
        description: String,
        video_type: String,
        video_data: PathBuf,
    }
    #[component]
    pub struct AudioComponent {
        name: String,
        description: String,
        r#type: String,
        audio_data: PathBuf,
    }
    #[component]
    pub struct ImageComponent {
        name: String,
        description: String,
        image_data: PathBuf,
    }

    #[nvproc::bincode_derive]
    #[nvproc::serde_derive]
    pub enum EGender {
        Male,
        Female,
        Custom(String),
    }
    //implement default for EGender
    impl Default for EGender {
        fn default() -> Self {
            Self::Male
        }
    }

    #[component]
    pub struct AgeComponent {
        age: f32,
    }
    #[component]
    pub struct GenderComponent {
        pub gender: EGender,
    }

    #[component]
    pub struct NameComponent {
        pub name: String,
        pub aliases: Vec<String>,
    }
    #[derive(Default)]
    #[nvproc::bincode_derive]
    #[nvproc::serde_derive]
    pub struct CharacterNameFormat {
        pub given_name: String,
        pub other_names: Vec<String>,
        pub family_name: String,
    }

    #[component]
    pub struct CharacterNameComponent {
        pub name: CharacterNameFormat,
        pub aliases: Vec<String>,
    }
    #[component]
    pub struct BinaryDatumComponent {
        data: PathBuf,
    }
    #[component]
    pub struct RelationshipComponent {
        pub relationship: relationship::Relationship,
    }
}
