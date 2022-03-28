use components_track::comp_link::COMPONENTS;

#[test]
fn test_component_linkme() {
    for comp in COMPONENTS.iter() {
        println!("{}", comp);
    }
    assert!(false);
}
