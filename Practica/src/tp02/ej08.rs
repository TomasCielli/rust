pub fn sumar_arreglos(a1: [f32; 5], a2: [f32; 5]) -> [f32; 5] {
    let mut resultado = [0.0; 5];
    for i in 0..5 {
        resultado[i] = a1[i] + a2[i];
    }
    resultado
}

#[test]

fn test_sumar_arreglos() {
    let arreglo1 = [1.0, 2.0, 3.0, 4.0, 5.0];
    let arreglo2 = [5.0, 4.0, 3.0, 2.0, 1.0];
    let resultado = sumar_arreglos(arreglo1, arreglo2);
    assert_eq!(resultado, [6.0, 6.0, 6.0, 6.0, 6.0]);
}
