pub fn longitud_de_cadenas(arreglo: &[String; 5]) -> [i32; 5] {
    let mut longitudes: [i32; 5] = [0; 5];
    for i in 0..5 {
        longitudes[i] = arreglo[i].len() as i32;
    }
    return longitudes;
}

#[test]

fn test_longitud_de_cadenas() {
    let arreglo = [
        String::from("Este"),
        String::from("es"),
        String::from("un"),
        String::from("ejemplo"),
        String::from("de test"),
    ];
    let longitudes = longitud_de_cadenas(&arreglo);
    assert_eq!(longitudes, [4, 2, 2, 7, 7]); // Longitudes de las cadenas
}
