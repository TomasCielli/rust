pub fn es_par(x: i32) -> bool {
    return x % 2 == 0;
}

#[test]
fn test_es_par() {
    assert_eq!(es_par(3), false);
    assert_eq!(es_par(4), true);
    assert_eq!(es_par(0), true);
}
