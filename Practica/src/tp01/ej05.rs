use std::io::stdin;

fn main() {
    let mut cadena: String = "Cadena escrita por el usuario: ".to_string();

    println!("");
    let mut cadena_usuario = String::new();
    println!("Escribe una cadena de texto: ");
    stdin()
        .read_line(&mut cadena_usuario)
        .expect("Error al leer la cadena");
    println!("");

    cadena.push_str(&cadena_usuario); //Une la cadena original con la cadena del usuario

    println!("");
    println!("{}", cadena);
    println!("");
}
