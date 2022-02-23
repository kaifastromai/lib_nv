use nvproc::Param;

use super::*;
#[derive(Clone, Param)]
pub struct TestParam {
    name: String,
}
pub fn test_fn(mir: &mut Mir, p: TestParam) -> Result<Box<TestRsrc>> {
    let name = "test";
    let mut rsrc = TestRsrc {
        name: name.to_string(),
    };
    println!("Hello world");
    Ok(Box::new(rsrc))
}

#[test]
fn test_action_register() {
    let action = Action::<TestRsrc, TestParam>::new_pure(
        &test_fn,
        TestParam {
            name: "test".to_string(),
        },
    );
    let mut mir = Mir::new();

    let mut act = Actman::new(&mut mir);
    act.register_action(action);
    act.advance();
    act.regress();
}
