pub fn cantidad_de_mayores(arreglo: [i32; 5], limite: i32) -> i32 {
    let mut contador = 0;
    for numero in arreglo {
        if numero > limite {
            contador += 1;
        }
    }
    contador
}

#[test]
fn test_cantidad_de_mayores() {
    let arreglo = [1, 2, 3, 4, 5];
    let limite = 3;
    let resultado = cantidad_de_mayores(arreglo, limite);
    assert_eq!(resultado, 2);
}
