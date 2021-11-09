use uuid::Uuid;

pub trait EntityAssociable {
    fn print(&self) {
        println!("Hello world");
    }
    fn get_id(&self) -> u64;
}
pub trait Temporal {
    fn print(&self) {
        println!("Temporal event!");
    }
}

pub struct Entity<'a> {
    _id: Uuid,
    fields: Option<&'a [Field<'a>]>,
    associatedData: Option<Vec<Box<dyn EntityAssociable>>>,
}

impl Entity<'_> {
    pub fn new(data: Vec<Box<dyn EntityAssociable>>) -> Self {
        Self {
            _id: Uuid::new_v4(),
            associatedData: Some(data),
            fields: None,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            associatedData: None,
            _id: Uuid::new_v4(),
            fields: None,
        }
    }
    ///Add a new associated data member to entity
    pub fn add(&mut self, item: Box<dyn EntityAssociable>) {
        self.associatedData.expect("Bad").push(item);
    }

    /// Get a reference to the entity's data.
    pub fn data(&self) -> &[Box<dyn EntityAssociable>] {
        self.associatedData.as_ref()
    }
}
pub struct Name<'a> {
    pub name: &'a str,
    pub aliases: &'a [&'a str],
}

struct Field<'a> {
    title: &'a str,
    value: &'a str,
}

impl<'a> Field<'a> {
    fn new(title: &'a str, value: &'a str) -> Self {
        Self { title, value }
    }
}

impl<'a> Name<'a> {
    pub fn new(name: &'a str, aliases: &'a [&'a str]) -> Self {
        Self { name, aliases }
    }
}
impl EntityAssociable for Name<'_> {}

pub struct Timeline {}
pub struct Arc {}
pub struct Scene {}
pub struct Location {}
pub struct Event {}
pub struct Image {}
pub struct Video {}
pub struct Audio {}
pub struct TextFile {}
pub struct Map {}
pub struct Progression<'a> {
    prev_progression: &'a Progression<'a>,
    next_progression: &'a Progression<'a>,
}
pub struct TextChunk<'a> {
    buffer: &'a String,
}
