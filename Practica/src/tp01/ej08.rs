use std::io::stdin;
fn main() {
    const CADENA: &str =
        "Esta cadena va a ser muy larga asi hay mas caracteres para contar jijijojo";
    let caracter: char;

    println!("");
    println!("Ingrese el caracter a contar: ");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    caracter = input.trim().parse().expect("Error al convertir a char");
    println!("");

    println!(
        "El caracter '{}' aparece {} veces en la cadena.",
        caracter,
        CADENA.matches(caracter).count()
    );
}
