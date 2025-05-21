fn main() {
    const ARREGLO1: [i32; 5] = [1, 2, 3, 4, 5];
    const ARREGLO2: [i32; 5] = [10, 20, 30, 40, 50];
    let arreglo3: [i32; 5];

    arreglo3 = [
        ARREGLO1[0] + ARREGLO2[0],
        ARREGLO1[1] + ARREGLO2[1],
        ARREGLO1[2] + ARREGLO2[2],
        ARREGLO1[3] + ARREGLO2[3],
        ARREGLO1[4] + ARREGLO2[4],
    ];

    println!("");
    println!("El arreglo 1 es: {:?}", ARREGLO1);
    println!("");

    println!("");
    println!("El arreglo 2 es: {:?}", ARREGLO2);
    println!("");

    println!("");
    println!("El arreglo 3 es: {:?}", arreglo3);
    println!("");
}
