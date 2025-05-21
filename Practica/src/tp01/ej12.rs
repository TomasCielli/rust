fn main() {
    const TUPLA: (&str, [i32; 5]) = ("Total de jugadores: ", [1, 2, 3, 4, 5]);

    let suma: i32 = TUPLA.1.iter().sum();

    println!("{}{}", TUPLA.0, suma);
}
