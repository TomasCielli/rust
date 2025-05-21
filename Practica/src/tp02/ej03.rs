pub fn suma_pares(numeros: [i32; 6]) -> i32 {
    let mut total = 0;
    for i in numeros {
        if i % 2 == 0 {
            // Si el numero es par
            total += i; //Lo suma al total
        }
    }
    total
}

#[test]
fn test_suma_pares() {
    let numeros = [1, 2, 3, 4, 5, 6];
    assert_eq!(suma_pares(numeros), 12); // 2 + 4 + 6 = 12
    let numeros_vacios: [i32; 6] = [0; 6];
    assert_eq!(suma_pares(numeros_vacios), 0); // Suma de un arreglo vacío es 0
    let numeros_sin_pares = [1, 3, 5, 7, 9, 11];
    assert_eq!(suma_pares(numeros_sin_pares), 0); // No hay números pares
}
