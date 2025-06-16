/*

1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de 
números primos. Cree un trait para la determinación del número primo e impleméntelo 
según corresponda. Utilice la función iter sobre el vector y aplique un closure para 
resolverlo. 

*/
#![allow(dead_code)]
pub trait Primos {

    fn es_primo(&self) -> bool;
}

impl Primos for i64 {

    fn es_primo(&self) -> bool {
        if *self <= 1 {
            return false;
        }
        if *self <= 3 {
            return true;
        }
        if *self % 2 == 0 || *self % 3 == 0 {
            return false;
        }

        let mut i = 5;
        while i * i <= *self {
            if *self % i == 0 || *self % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}


fn cant_primos(numeros: Vec<i64>) -> usize {
    numeros.iter().filter(|x| x.es_primo()).count()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cant_primos () {
        let numeros = vec![-1, 0, 1, 2, 3, 5, 6, 11];
        assert_eq!(cant_primos(numeros), 4);
    }

    #[test]
    fn test_cant_primos_vacio () {
        let numeros = Vec::new();
        assert_eq!(cant_primos(numeros), 0);
    }

}