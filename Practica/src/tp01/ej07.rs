fn main() {
    const CONSTANTE: i32 = 5; //Constante multiplicadora

    let arreglo = [1, 10, 100, 1000, 10000, 100000]; //Arreglo base

    let arreglo_multiplicado = arreglo.map(|x| x * CONSTANTE); //El map multiplica los campos y crea un nuevo arreglo

    println!(
        "Arreglo: {:?} | Arreglo x {}: {:?}",
        arreglo, CONSTANTE, arreglo_multiplicado
    );
}
