pub fn cantidad_de_cadenas_mayor_a(arreglo: &[String], limite: i32) -> i32 {
    let mut contador = 0;
    for cadena in arreglo {
        if cadena.len() as i32 > limite {
            contador += 1;
        }
    }
    contador
}

#[test]

fn test_cantidad_de_cadenas_mayor_a() {
    let arreglo = [
        String::from("Hola"),
        String::from("mundo"),
        String::from("esto"),
        String::from("es"),
        String::from("Rust"),
    ];
    let limite = 3;
    let resultado = cantidad_de_cadenas_mayor_a(&arreglo, limite);
    assert_eq!(resultado, 4);
}
