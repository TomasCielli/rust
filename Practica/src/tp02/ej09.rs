pub fn cantidad_en_rango(arreglo: [i32; 5], inferior: i32, superior: i32) -> i32 {
    let mut contador = 0;
    for numero in arreglo {
        if numero >= inferior && numero <= superior {
            contador += 1;
        }
    }
    contador
}

#[test]
fn test_cantidad_en_rango() {
    let arreglo = [1, 2, 3, 4, 5];
    let inferior = 2;
    let superior = 4;
    let resultado = cantidad_en_rango(arreglo, inferior, superior);
    assert_eq!(resultado, 3);
}
