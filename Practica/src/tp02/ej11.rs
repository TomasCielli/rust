pub fn multiplicar_valores(arreglo: &mut [i32; 5], factor: i32) {
    for i in 0..5 {
        arreglo[i] *= factor;
    }
}

#[test]
fn test_multiplicar_valores() {
    let mut arreglo = [1, 2, 3, 4, 5];
    let factor = 2;
    multiplicar_valores(&mut arreglo, factor);
    assert_eq!(arreglo, [2, 4, 6, 8, 10]);
}
