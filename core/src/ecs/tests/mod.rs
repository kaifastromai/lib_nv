#[test]
fn test_file_to_static_string() {
    let static_string = nvproc::file_to_static_string!("/home/jstrom/dev/test.txt");
    assert_eq!(static_string, "HelloWorld\n");
}

