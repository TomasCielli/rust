use std::io::stdin;
fn main() {
    const ENCONTRADO: &str = "La cadena se encuentra en el arreglo";

    const ARREGLO: [&str; 5] = ["Juan", "Pedro", "Maria", "Tomas", "Ana"];
    let encontrado: bool;

    let mut cadena = String::new();
    println!("Ingrese una cadena:");
    stdin()
        .read_line(&mut cadena)
        .expect("Error al leer la cadena");

    encontrado = ARREGLO.iter().any(|&x| x == cadena.trim());

    if encontrado {
        println!("{}", ENCONTRADO);
    } else {
        println!("La cadena no se encuentra en el arreglo");
    }
}
