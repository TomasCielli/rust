pub fn reemplazar_pares(arreglo: &mut [i32; 5]) {
    for i in 0..5 {
        if arreglo[i] % 2 == 0 {
            arreglo[i] = -1;
        }
    }
}

#[test]

fn test_reemplazar_pares() {
    let mut arreglo = [1, 2, 3, 4, 5];
    reemplazar_pares(&mut arreglo);
    assert_eq!(arreglo, [1, -1, 3, -1, 5]);
}
