pub trait EntityAssociable {
    fn print(&self) {
        println!("Hello world");
    }
}

pub struct Entity {
    data: Vec<Box<dyn EntityAssociable>>,
}

impl Entity {
    pub fn new(data: Vec<Box<dyn EntityAssociable>>) -> Self {
        Self { data }
    }
    pub fn new_empty() -> Self {
        let v: Vec<Box<dyn EntityAssociable>> = Vec::new();
        Self { data: v }
    }
    ///Add a new associated data member to entity
    pub fn add(&mut self, item: Box<dyn EntityAssociable>) {
        self.data.push(item);
    }

    /// Get a reference to the entity's data.
    pub fn data(&self) -> &[Box<dyn EntityAssociable>] {
        self.data.as_ref()
    }
}
pub struct Name<'a> {
    pub name: &'a str,
}
impl EntityAssociable for Name<'_> {}

impl EntityAssociable for Entity {}
