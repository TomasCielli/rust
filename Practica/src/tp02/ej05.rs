pub fn duplicar_valores(a: [f32; 5]) -> [f32; 5] {
    let mut b = [0.0; 5];
    for i in 0..5 {
        b[i] = a[i] * 2.0;
    }
    b
}

#[test]
fn test_duplicar_valores() {
    let a = [1.0, 2.0, 3.0, 4.0, 5.0];
    let resultado = duplicar_valores(a);
    assert_eq!(resultado, [2.0, 4.0, 6.0, 8.0, 10.0]);
    let a_vacio = [0.0; 5];
    let resultado_vacio = duplicar_valores(a_vacio);
    assert_eq!(resultado_vacio, [0.0; 5]);
    let a_negativos = [-1.0, -2.0, -3.0, -4.0, -5.0];
    let resultado_negativos = duplicar_valores(a_negativos);
    assert_eq!(resultado_negativos, [-2.0, -4.0, -6.0, -8.0, -10.0]);
}
