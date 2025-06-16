impl PartialEq for Fecha{
    
    fn eq(&self, otro: &Self) -> bool{
        return (self.anio == otro.anio) && (self.mes == otro.mes) && (self.dia == otro.dia)
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Fecha {
    pub dia: u8,
    pub mes: u8,
    pub anio: u16,
}


impl Fecha {
    pub fn new(dia: u8, mes: u8, anio: u16) -> Fecha {
        Fecha { dia, mes, anio }
    }

    pub fn es_bisiesto(&self) -> bool {
        if (self.anio % 4 == 0) && !((self.anio % 100 == 0) && (self.anio % 400 != 0)) {
            return true;
        };
        return false;
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes > 12 || self.mes < 1 {
            return false;
        }
        if self.dia > 31 || self.dia < 1 {
            return false;
        }
        if self.mes == 2 && self.dia > 29 {
            return false;
        }
        if self.mes == 2 && self.dia == 29 && !self.es_bisiesto() {
            return false;
        }
        if (self.mes == 4 || self.mes == 6 || self.mes == 9 || self.mes == 11) && self.dia > 30 {
            return false;
        }
        return true;
    }

    pub fn sumar_dias(&mut self, dias: u8) -> bool {
        let mut fecha = self.clone();
        for _ in 0..dias {
            fecha.dia += 1;
            if fecha.mes == 2 && fecha.dia > 28 {
                //Si el mes es febrero
                if fecha.es_bisiesto() && fecha.dia > 29 {
                    // Si el año es bisiesto y ya pasamos el 29
                    fecha.dia = 1; // Vuelvo al dia 1
                    fecha.mes += 1; // Paso al mes 3
                } else if !fecha.es_bisiesto() {
                    //Si no es bisiesto
                    fecha.dia = 1;
                    fecha.mes += 1;
                }
            } else if (fecha.mes == 4 || fecha.mes == 6 || fecha.mes == 9 || fecha.mes == 11)
                && fecha.dia > 30
            {
                // Si es un mes de 30 dias y ya pase el  dia 30
                fecha.dia = 1; // Vuelvo al dia 1
                fecha.mes += 1; // Paso al mes siguiente
            } else if fecha.dia > 31 {
                fecha.dia = 1;
                fecha.mes += 1;
            }
            if fecha.mes > 12 {
                //Si complete el año
                fecha.mes = 1; //Vuelvo al mes 1
                fecha.anio += 1; //Aumento el año
            }
        }
        if fecha.es_fecha_valida() {
            self.anio = fecha.anio;
            self.mes = fecha.mes;
            self.dia = fecha.dia;
            return true;
        }
        return false;
    }

    pub fn restar_dias(&mut self, dias: u8) -> bool {
        let mut fecha = self.clone(); // Clonamos la fecha original
        for _ in 0..dias {
            fecha.dia -= 1;
            if fecha.dia == 0 {
                // Si no quedan dias en el mes
                fecha.mes -= 1; // Paso al mes anterior
                if fecha.mes == 0 {
                    // Si no quedan meses en el año
                    fecha.mes = 12; // Paso al mes 12
                    fecha.anio -= 1; //Paso al año anterior
                }
                if fecha.mes == 2 {
                    //Si el mes es febrero
                    if fecha.es_bisiesto() {
                        //Y el año es bisiesto
                        fecha.dia = 29; //Paso al dia 29
                    } else {
                        //Si no es bisiesto
                        fecha.dia = 28; //Paso al dia 28
                    }
                } else if fecha.mes == 4 || fecha.mes == 6 || fecha.mes == 9 || fecha.mes == 11 {
                    // Si es un mes de 30 dias
                    fecha.dia = 30;
                } else {
                    fecha.dia = 31;
                }
            }
        }
        if fecha.es_fecha_valida() {
            self.anio = fecha.anio;
            self.mes = fecha.mes;
            self.dia = fecha.dia;
            return true;
        } else {
            return false;
        } // Si la fecha no es valida, devuelve None
    }

    /*
    pub fn es_mayor(&self, una_fecha: Fecha) -> bool {

        if self.anio > una_fecha.anio {  //Si el año es mayor que la fecha a comparar directamente devuelve true
            return true;
        }
        if self.anio == una_fecha.anio {  // Si tienen el mismo año
            if self.mes > una_fecha.mes { // Y el mes es mayor que la fecha a comparar devuelve true
                return true;
            }
            if self.mes == una_fecha.mes {    // Si tienen el mismo mes y el mismo año
                if self.dia > una_fecha.dia { // Y el dia es mayor que la fecha a comparar devuelve true
                    return true;
                }
            }
        }
        return false; //Si no se cumple ninguna de las anteriores devuelve false
    }
    */

    pub fn es_mayor_2(&self, una_fecha: Fecha) -> bool {
        if (self.anio > una_fecha.anio)
            || ((self.anio == una_fecha.anio) && (self.mes > una_fecha.mes))
            || ((self.anio == una_fecha.anio)
                && (self.mes == una_fecha.mes)
                && (self.dia > una_fecha.dia))
        {
            return true;
        }
        return false;
    }

    pub fn mostrar(&self) -> String {
        format!("{:02}/{:02}/{}", self.dia, self.mes, self.anio) //:02 agrega un 0 adelante si tiene un solo digito
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha() {
        let fecha = Fecha::new(1, 1, 2023);
        assert_eq!(fecha.dia, 1);
        assert_eq!(fecha.mes, 1);
        assert_eq!(fecha.anio, 2023);
    }

    #[test]
    fn test_es_bisiesto() {
        let fecha = Fecha::new(1, 1, 2020);
        assert_eq!(fecha.es_bisiesto(), true);
    }

    #[test]
    fn test_no_es_bisiesto() {
        let fecha = Fecha::new(1, 1, 2023);
        assert_eq!(fecha.es_bisiesto(), false);
    }

    #[test]
    fn test_fecha_valida() {
        let fecha = Fecha::new(29, 2, 2020); //Año bisiesto
        assert_eq!(fecha.es_fecha_valida(), true);
    }

    #[test]
    fn test_fecha_no_valida() {
        let fecha = Fecha::new(29, 2, 2023);
        assert_eq!(fecha.es_fecha_valida(), false);
    }

    #[test]
    fn test_sumar_dias() {
        let mut fecha = Fecha::new(28, 2, 2020); //Año bisiesto
        fecha.sumar_dias(1);
        assert_eq!(fecha.dia, 29);
        assert_eq!(fecha.mes, 2);
        assert_eq!(fecha.anio, 2020);
    }

    #[test]
    fn test_restar_dias() {
        let mut fecha = Fecha::new(1, 3, 2020); //Año bisiesto
        fecha.restar_dias(1);
        assert_eq!(fecha.dia, 29);
        assert_eq!(fecha.mes, 2);
        assert_eq!(fecha.anio, 2020);
    }

    #[test]
    fn test_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2023);
        let fecha2 = Fecha::new(1, 1, 2022);
        assert_eq!(fecha1.es_mayor_2(fecha2), true);
    }

    #[test]
    fn test_no_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2022);
        let fecha2 = Fecha::new(1, 1, 2023);
        assert_eq!(fecha1.es_mayor_2(fecha2), false);
    }
}
