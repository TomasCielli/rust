pub fn ordenar_nombres(nombres: &mut [String; 5]) {
    nombres.sort_by(|a, b| a.cmp(b));
}

#[test]

fn test_ordenar_nombres() {
    let mut nombres = [
        String::from("Tomas"),
        String::from("Ana"),
        String::from("Pedro"),
        String::from("Maria"),
        String::from("Juan"),
    ];
    ordenar_nombres(&mut nombres);
    assert_eq!(
        nombres,
        [
            String::from("Ana"),
            String::from("Juan"),
            String::from("Maria"),
            String::from("Pedro"),
            String::from("Tomas")
        ]
    );
}
