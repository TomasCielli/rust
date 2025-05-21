//ESTRUCTURA COLOR
enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

//IMPLEMENTACION COLOR
impl Color {
    fn devolver_color(&self) -> String {
        match self {
            Color::ROJO => return "Rojo".to_string(),
            Color::VERDE => return "Verde".to_string(),
            Color::AZUL => return "Azul".to_string(),
            Color::AMARILLO => return "Amarillo".to_string(),
            Color::BLANCO => return "Blanco".to_string(),
            Color::NEGRO => return "Negro".to_string(),
        }
    }

    //Para saber si es un color primario
    fn soy_primario(&self) -> bool {
        if self.devolver_color() == "Rojo"
            || self.devolver_color() == "Verde"
            || self.devolver_color() == "Azul"
        {
            return true;
        }
        return false;
    }
}

//ESTRUCTURA AUTO
struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f64,
    color: Color,
}

impl Auto {
    fn new(marca: String, modelo: String, anio: u32, precio_bruto: f64, color: Color) -> Auto {
        Auto {
            marca,
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }

    fn calcular_precio(&self) -> f64 {
        const ANIODESCUENTO: u32 = 2000;
        const RECARGOCOLOR: f64 = 0.25;
        const DESCUENTOCOLOR: f64 = 0.10;
        const MARCA: &str = "BMW";
        const RECARGOMARCA: f64 = 0.15;
        const DESCUENTOPORANIO: f64 = 0.05;

        let mut total: f64 = self.precio_bruto;
        if self.color.soy_primario() {
            total += total * RECARGOCOLOR;
        } else {
            total -= total * DESCUENTOCOLOR;
        }
        if self.marca == MARCA.to_string() {
            total += total * RECARGOMARCA;
        }
        if self.anio < ANIODESCUENTO {
            total -= total * DESCUENTOPORANIO;
        }

        return total;
    }
}
//Tuple struct, el usize como rango para q me diga la dimF, y el vector de autos
struct VectorAutos(usize, Vec<Auto>);

//ESTRUCTURA CONCESIONARIO
struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    autos: VectorAutos,
}

//IMPLEMENTACION CONCESIONARIO
impl ConcesionarioAuto {
    fn new(nombre: String, direccion: String, x: usize) -> ConcesionarioAuto {
        let v: Vec<Auto> = Vec::new();
        let ts: VectorAutos = VectorAutos(x, v);
        ConcesionarioAuto {
            nombre,
            direccion,
            autos: ts,
        }
    }

    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.1.len() < self.autos.0 {
            self.autos.1.push(auto);
            return true;
        } else {
            return false;
        }
    }

    fn eliminar_auto(&mut self, auto: Auto) {
        if let Some(pos) = self.autos.1.iter().position(|x| {
            auto.marca == x.marca
                && auto.modelo == x.modelo
                && auto.anio == x.anio
                && auto.precio_bruto == x.precio_bruto
                && auto.color.devolver_color() == x.color.devolver_color()
        }) {
            self.autos.1.swap_remove(pos);
        }
    }

    fn buscar_auto(&self, auto: Auto) -> Option<&Auto> {
        self.autos.1.iter().find(|x| {
            auto.marca == x.marca
                && auto.modelo == x.modelo
                && auto.anio == x.anio
                && auto.precio_bruto == x.precio_bruto
                && auto.color.devolver_color() == x.color.devolver_color()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_concesionario() {
        let concesionario =
            ConcesionarioAuto::new("Manolo".to_string(), "Calle 73".to_string(), 10);
        assert_eq!(concesionario.nombre, "Manolo".to_string());
        assert_eq!(concesionario.direccion, "Calle 73".to_string());
        assert_eq!(concesionario.autos.0, 10);
    }

    #[test]
    fn test_crear_auto() {
        let auto = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        );
        assert_eq!(auto.marca, "BMW".to_string());
        assert_eq!(auto.modelo, "X5".to_string());
        assert_eq!(auto.anio, 2020);
        assert_eq!(auto.precio_bruto, 50000.0);
        assert_eq!(auto.color.devolver_color(), "Rojo".to_string());
    }

    #[test]
    fn test_calcular_precio() {
        let auto = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        );
        assert_eq!(auto.calcular_precio(), 50000.0 * 1.25 * 1.15); // 25% recargo por color primario y 15% recargo por marca BMW
    }

    #[test]

    fn test_agregar_auto() {
        let mut concesionario =
            ConcesionarioAuto::new("Manolo".to_string(), "Calle 73".to_string(), 1);
        let auto = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        );
        assert_eq!(concesionario.agregar_auto(auto), true);
        assert_eq!(concesionario.autos.1.len(), 1);

        let auto2 = Auto::new(
            "Audi".to_string(),
            "Q5".to_string(),
            2021,
            60000.0,
            Color::VERDE,
        );
        assert_eq!(concesionario.agregar_auto(auto2), false);
        assert_eq!(concesionario.autos.1.len(), 1);
    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionario =
            ConcesionarioAuto::new("Manolo".to_string(), "Calle 73".to_string(), 1);
        let auto = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        );
        concesionario.agregar_auto(auto);
        assert_eq!(concesionario.autos.1.len(), 1);

        concesionario.eliminar_auto(Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        ));
        assert_eq!(concesionario.autos.1.len(), 0);
    }

    #[test]
    fn test_buscar_auto() {
        let mut concesionario =
            ConcesionarioAuto::new("Manolo".to_string(), "Calle 73".to_string(), 1);
        let auto = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        );
        concesionario.agregar_auto(auto);

        let resultado = concesionario.buscar_auto(Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        ));
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().marca, "BMW".to_string());
    }

    #[test]
    fn test_no_encontrar_auto() {
        let mut concesionario =
            ConcesionarioAuto::new("Manolo".to_string(), "Calle 73".to_string(), 1);
        let auto = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::ROJO,
        );
        concesionario.agregar_auto(auto);

        let resultado = concesionario.buscar_auto(Auto::new(
            "Audi".to_string(),
            "Q5".to_string(),
            2021,
            60000.0,
            Color::VERDE,
        ));
        assert!(resultado.is_none());
    }
}
