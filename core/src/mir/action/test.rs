use super::actions::*;
use super::*;

#[test]
fn action_test_add() {
    let mut em = EntityManager::new();
    let mut action_stack = ActionStack::new(10);
    let mut action = Box::new(AddEntityAction::new(&mut em, String::from("Default")).unwrap());
    action_stack.add_action(Action::Undoable(action));
    assert_eq!(1, action_stack.get_actions().len());
    action_stack.undo().unwrap();
}

#[allow(mutable_transmutes)]
#[test]
fn action_delete() {
    let mut em = EntityManager::new();
    let mut action_stack = ActionStack::new(10);
    let id = em.create_entity(String::from("Default"));
    let mut action = Box::new(
        DeleteEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            id,
        )
        .unwrap(),
    );

    action_stack.add_action(Action::Undoable(action));
    assert_eq!(1, action_stack.get_actions().len());

    assert_eq!(0, em.get_all_entities().len());

    action_stack.undo().unwrap();
    assert_eq!(1, em.get_all_entities().len());
}

#[allow(mutable_transmutes)]
#[test]
fn action_multi_stack() {
    let mut em = EntityManager::new();
    let mut action_stack = ActionStack::new(10);
    let mut add_action = Box::new(
        AddEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            String::from("Default"),
        )
        .unwrap(),
    );
    let mut add_action2 = Box::new(
        AddEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            String::from("Default"),
        )
        .unwrap(),
    );

    let mut add_action3 = Box::new(
        AddEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            String::from("Default"),
        )
        .unwrap(),
    );

    //undo each action
    action_stack.add_action(Action::Undoable(add_action));

    let id = em.create_entity(String::from("Default"));

    let mut delete_action = Box::new(
        DeleteEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            id,
        )
        .unwrap(),
    );
    action_stack.add_action(Action::Undoable(add_action2));
    action_stack.add_action(Action::Undoable(add_action3));
    action_stack.add_action(Action::Undoable(delete_action));
    assert_eq!(4, action_stack.get_actions().len());
    action_stack.undo().unwrap();
    action_stack.undo().unwrap();
    action_stack.undo().unwrap();
    action_stack.undo().unwrap();

    //the em should be empty
    assert_eq!(1, em.get_all_entities().len());
}

#[allow(mutable_transmutes)]
#[test]
fn action_invalidate_stack() {
    let mut em = EntityManager::new();
    let mut action_stack = ActionStack::new(10);
    let mut add_action = Box::new(
        AddEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            String::from("Default"),
        )
        .unwrap(),
    );
    let mut add_action2 = Box::new(
        AddEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            String::from("Default"),
        )
        .unwrap(),
    );

    let mut add_action3 = Box::new(
        AddEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            String::from("Default"),
        )
        .unwrap(),
    );

    let id = em.create_entity(String::from("Default"));
    let mut delete_action = Box::new(
        DeleteEntityAction::new(
            unsafe {
                let em_ref = &em;
                let em_ref_mut = std::mem::transmute::<&EntityManager, &mut EntityManager>(em_ref);
                em_ref_mut
            },
            id,
        )
        .unwrap(),
    );
    //undo each action
    action_stack.add_action(Action::Undoable(add_action));
    action_stack.add_action(Action::Undoable(add_action2));
    action_stack.add_action(Action::Undoable(delete_action));
    action_stack.undo().unwrap();
    action_stack.add_action(Action::Undoable(add_action3));
    action_stack.undo().unwrap();
    action_stack.undo().unwrap();
    action_stack.undo().unwrap();

    //the em should be empty
    assert_eq!(1, em.get_all_entities().len());
}
