// use super::*;
// use archetypes::*;

// #[test]
// fn legion_add_component() {
//     let mut em = EntityManager::new();
//     let entity = em.add_entity();
//     em.get_world_mut()
//         .entry(entity)
//         .unwrap()
//         .add_component(("Hello"));
//     assert_eq!(
//         em.get_world_mut()
//             .entry(entity)
//             .unwrap()
//             .get_component::<(&str)>()
//             .unwrap(),
//         ("Hello")
//     )
// }
