struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: u64,
}

impl Producto {
    fn new(nombre: String, precio_bruto: f64, id: u64) -> Producto {
        Producto {
            nombre,
            precio_bruto,
            id,
        }
    }

    fn calcular_impuestos(&self, porcentaje_de_impuestos: f64) -> f64 {
        (self.precio_bruto * porcentaje_de_impuestos) / 100.0
    }

    fn aplicar_descuento(&self, porcentaje_de_descuento: f64) -> f64 {
        ((self.precio_bruto * porcentaje_de_descuento) / 100.0) * -1.0
    }

    fn calcular_precio_total(&self, impuesto: Option<f64>, descuento: Option<f64>) -> f64 {
        let mut total: f64 = self.precio_bruto;

        if let Some(imp) = impuesto {
            total += self.calcular_impuestos(imp);
        }

        if let Some(desc) = descuento {
            total += self.aplicar_descuento(desc);
        }

        return total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producto() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(producto.nombre, "Ravioles");
        assert_eq!(producto.precio_bruto, 100.0);
        assert_eq!(producto.id, 1);
    }

    #[test]
    fn test_calcular_impuestos() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(producto.calcular_impuestos(21.0), 21.0);
    }

    #[test]
    fn test_aplicar_descuento() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(producto.aplicar_descuento(10.0), -10.0);
    }

    #[test]
    fn test_calcular_precio_total() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(
            producto.calcular_precio_total(Some(10.0), Some(10.0)),
            100.0
        );
    }

    #[test]
    fn test_calcular_precio_total_sin_impuesto_y_sin_descuento() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(producto.calcular_precio_total(None, None), 100.0);
    }

    #[test]
    fn test_calcular_precio_total_con_impuesto_sin_descuento() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(producto.calcular_precio_total(Some(21.0), None), 121.0);
    }

    #[test]
    fn test_calcular_precio_total_sin_impuesto_con_descuento() {
        let producto = Producto::new("Ravioles".to_string(), 100.0, 1);
        assert_eq!(producto.calcular_precio_total(None, Some(10.0)), 90.0);
    }
}
