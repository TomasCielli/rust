fn main() {
    const ARREGLO: [i32; 5] = [1, 2, 3, 4, 5];
    let suma: i32 = ARREGLO.iter().sum();

    println!("La suma de {:?} = {}", ARREGLO, suma);
}
