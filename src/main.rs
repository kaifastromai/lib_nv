pub mod core;

fn main() {
    let n = core::Name { name: "Bob" };
    let mut c: core::Entity = core::Entity::new_empty();
    let mut c2: core::Entity = core::Entity::new_empty();

    c.add(Box::new(n));
    c.add(Box::new(c2));
    for v in c.data() {
        v.print();
    }
}
