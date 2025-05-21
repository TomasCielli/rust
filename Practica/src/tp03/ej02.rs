struct Rectangulo {
    longitud: f64,
    ancho: f64,
}

impl Rectangulo {
    fn new(longitud: f64, ancho: f64) -> Rectangulo {
        Rectangulo { longitud, ancho }
    }

    fn calcular_area(&self) -> f64 {
        return self.longitud * self.ancho;
    }

    fn calcular_perimetro(&self) -> f64 {
        return self.longitud * 2.0 + self.ancho * 2.0;
    }

    fn es_cuadrado(&self) -> bool {
        return self.longitud == self.ancho;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn test_rectangulo() {
        let rectangulo = Rectangulo::new(4.0, 2.0);

        assert_eq!(rectangulo.calcular_area(), 8.0);
        assert_eq!(rectangulo.calcular_perimetro(), 12.0);
        assert_eq!(rectangulo.es_cuadrado(), false);
    }

    #[test]
    fn test_cuadrado() {
        let cuadrado = Rectangulo::new(2.0, 2.0);

        assert_eq!(cuadrado.calcular_area(), 4.0);
        assert_eq!(cuadrado.calcular_perimetro(), 8.0);
        assert_eq!(cuadrado.es_cuadrado(), true);
    }
}
