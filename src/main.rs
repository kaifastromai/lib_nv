fn main() {
    println!("Hello, world!");
}

struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn can_hold(&self, var: u32) -> bool {
        var > self.width
    }
}
#[test]
fn larger() {
    let r = Rect {
        width: 30,
        height: 30,
    };
    assert!(r.can_hold(20));
}
