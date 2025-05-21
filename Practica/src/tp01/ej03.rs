use std::io::stdin;

fn main() {
    const BOOLEANA: bool = true;
    let variable_leida: bool;

    println!("");
    println!("La variable es: {}", BOOLEANA);
    println!("");

    println!("");
    println!("Escribe un valor booleano (true o false):");
    let mut variable_bool = String::new();
    println!("");

    stdin()
        .read_line(&mut variable_bool)
        .expect("Error al leer booleano");
    variable_leida = variable_bool
        .trim()
        .parse()
        .expect("Error al convertir a booleano");

    println!("");
    println!(
        "{} and {} = {}",
        BOOLEANA,
        variable_leida,
        BOOLEANA && variable_leida
    );
    println!("");

    println!("");
    println!(
        "{} or {} = {}",
        BOOLEANA,
        variable_leida,
        BOOLEANA || variable_leida
    );
    println!("");
}
