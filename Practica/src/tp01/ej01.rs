use std::io::stdin;

fn main() {
    const X: f32 = 10.0; // numero flotante, no necesita ser mutable
    let  num: i8; // numero entero, no necesita ser mutable

    println!("Introduce un número entero: ");
    
    let mut numero = String::new();                                                   //lectura del numero
    stdin().read_line(&mut numero).expect("Error al leer el número");                   //hay que hacerlo como string
    num = numero.trim().parse().expect("Error al convertir el número");                   //y dsp pasarlo a i8

    println!("");
    println!("El número introducido es: {}", num);            //donde ponga las {} se cambia por el valor de la variable en el orden que esten
    println!("");

    println!("");
    println!("{} multiplicado por {} es: {}", num, X, num as f32 * X);  //con el as f32 lo pasamos a flotante sin modificar el valor
    println!("");

    println!("");
    println!("{} dividido entre {} es: {}", num, X, num as f32 / X);
    println!("");

    println!("");
    println!("{} sumado a {} es: {}", num, X, num as f32 + X);
    println!("");

    println!("");
    println!("{} restado a {} es: {}", num, X, num as f32 - X);
}
