use super::*;

#[test]
fn test_register_action() {
    let mut mir=Mir::new();
    let c = |m: &mut Mir, k: &mut dyn ResrcTy| {
        print!("Hello");
        return true;
    };
    let action = Action::new(0, &action_add_entity);

    let mut act=Actman::new(&mut mir);
    act.register_action(action)


}
