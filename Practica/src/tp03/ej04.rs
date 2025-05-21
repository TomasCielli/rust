enum TipoTriangulo {
    EQUILATERO,
    ISOCELES,
    ESCALENO,
}

impl TipoTriangulo {
    pub fn mostrar_tipo(&self) -> String {
        match self {
            TipoTriangulo::EQUILATERO => "Equilatero".to_string(),
            TipoTriangulo::ESCALENO => "Escaleno".to_string(),
            TipoTriangulo::ISOCELES => "Isoceles".to_string(),
        }
    }
}

struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
    tipo: TipoTriangulo,
}

impl Triangulo {
    fn new(lado1: f64, lado2: f64, lado3: f64) -> Triangulo {
        Triangulo {
            lado1,
            lado2,
            lado3,
            tipo: Triangulo::determinar_tipo(lado1, lado2, lado3), ///////////////
        }
    }

    pub fn determinar_tipo(lado1: f64, lado2: f64, lado3: f64) -> TipoTriangulo {
        if (lado1 == lado2) || (lado1 == lado3) || (lado2 == lado3) {
            if (lado1 == lado2) && (lado2 == lado3) {
                return TipoTriangulo::EQUILATERO;
            } else {
                return TipoTriangulo::ISOCELES;
            }
        }
        return TipoTriangulo::ESCALENO;
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }

    pub fn calcular_area(&self) -> f64 {
        fn area_isoceles(base: f64, altura: f64) -> f64 {
            base * altura / 2.0
        }

        fn area_equilatero(lado: f64) -> f64 {
            (lado * lado * f64::sqrt(3.0)) / 4.0
        }

        fn area_escaleno(lado1: f64, lado2: f64, lado3: f64) -> f64 {
            let s = (lado1 + lado2 + lado3) / 2.0;
            return f64::sqrt(s * (s - lado1) * (s - lado2) * (s - lado3));
        }

        match self.tipo {
            TipoTriangulo::ISOCELES => {
                if self.lado1 == self.lado2 {
                    return area_isoceles(self.lado1, self.lado3);
                } else {
                    return area_isoceles(self.lado1, self.lado2);
                }
            }

            TipoTriangulo::EQUILATERO => area_equilatero(self.lado1),

            TipoTriangulo::ESCALENO => area_escaleno(self.lado1, self.lado2, self.lado3),
        }
    }

    pub fn mostrar_tipo(&self) -> String {
        self.tipo.mostrar_tipo()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_triangulo() {
        let triangulo = Triangulo::new(10.0, 10.0, 10.0);

        assert_eq!(triangulo.lado1, 10.0);
        assert_eq!(triangulo.lado2, 10.0);
        assert_eq!(triangulo.lado3, 10.0);
        assert_eq!(triangulo.mostrar_tipo(), "Equilatero".to_string());
    }
}
