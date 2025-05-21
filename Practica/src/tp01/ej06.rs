use std::io::stdin;

fn main() {
    const NUM: i32 = 44;
    let numero_leido: i32;

    println!("");
    println!("Escriba un numero:");
    let mut x = String::new();
    stdin().read_line(&mut x).expect("Error al leer el numero");
    numero_leido = x.trim().parse().expect("Error al convertir el numero");
    println!("");

    println!("");
    println!(
        "({} + {}) x ({} + {}) = {}",
        NUM,
        numero_leido,
        NUM,
        numero_leido,
        (NUM + numero_leido) * (NUM + numero_leido)
    );
}
