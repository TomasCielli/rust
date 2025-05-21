pub fn cantidad_impares(numeros: [i32; 6]) -> i32 {
    let mut total: i32 = 0;
    for i in 0..numeros.len() {
        if numeros[i] % 2 != 0 {
            total += 1;
        }
    }
    return total;
}

#[test]
fn test_cantidad_impares() {
    let numeros = [1, 2, 3, 4, 5, 6];
    assert_eq!(cantidad_impares(numeros), 3); // 1 + 3 + 5 = 3
    let numeros_vacios: [i32; 6] = [0; 6];
    assert_eq!(cantidad_impares(numeros_vacios), 0); // Suma de un arreglo vacío es 0
    let numeros_sin_impares = [2, 4, 6, 8, 10, 12];
    assert_eq!(cantidad_impares(numeros_sin_impares), 0); // No hay números impares
}
