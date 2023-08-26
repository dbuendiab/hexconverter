#[cfg(test)]
mod tests {

    // use crate::Numero;               // Cómo referir el módulo desde el propio fichero en /src

    use hexconverter::numero::Numero;   // Cómo referir el módulo desde el directorio /test

    fn primitiva(face: &str, base: usize, valor: usize, error: &str) {
        //let face = face;
        //let base = base;
        let n = Numero::new(face.to_owned(), base);
        match n {
            Ok(numero) => {
                assert_eq!(numero.value(), valor);
            }, 
            Err(cadena) => {
                assert_eq!(cadena, error.to_owned());
            }
        }
    }


    #[test]
    fn numero_bien_b2() {
        primitiva("10010110011", 2_usize, 1203, "Esto no da error");
    }

    #[test]
    fn numero_err_b2() {
        primitiva("2010010", 2_usize, 0, "El carácter '2' no es válido en base 2");
    }

    #[test]
    fn numero_bien_b35() {
        primitiva("diegobuendia", 35_usize, 1305933913122261440, "");
    }

    #[test]
    fn numero_bien_usize_max() {
        primitiva(&(usize::MAX).to_string(), 10_usize, 18446744073709551615, "sfdgsgfsdf");
    }

    #[test]
    fn numero_error_base_1() {
        primitiva(&(usize::MAX).to_string(), 1_usize, 0, "Base fuera de límites");
    }

    #[test]
    fn numero_error_base_36() {
        primitiva(&(usize::MAX).to_string(), 36_usize, 0, "Base fuera de límites");
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Conversión de bases 
    fn primitiva2(face: &str, base: usize, base2: usize, valor: &str, error: &str) {

        // Generar el número en base original
        let rn = Numero::new(face.to_owned(), base);
        match rn {
            Ok(n) => {
                // Ejecutar la conversión de base
                match n.to_base(base2) {
                    // Si todo bien, comprobar el resulta do de la base
                    Ok(n2) => {
                        assert_eq!(n2.face(), &valor.to_owned());
                    },
                    Err(cadena) => {
                        assert_eq!(cadena, error.to_owned());
                    }
                }
            },
            Err(cadena) => {
                assert_eq!(cadena, error.to_owned());
            }
        }      
    }

    fn primitiva3(face: &str, base: usize, base2: usize, valor: &str) {
        // Generar el número en base original
        let n1 = Numero::new(face.to_owned(), base).unwrap();
    
        // Ejecutar la conversión de base
        let n2 = n1.to_base(base2).unwrap();
    
        // Verificar el resultado de la conversión
        assert_eq!(n2.face(), &valor.to_owned());
    }

    #[test]
    fn numero_convert_b2_8() {
        primitiva3("10010110011", 2_usize, 8_usize, "2263");
    }

    #[test]
    fn numero_convert_b8_2() {
        primitiva3("2263", 8_usize, 2_usize, "10010110011");
    }

    #[test]
    fn numero_convert_error_digito() {
        primitiva2("2263", 2_usize, 8_usize, "1001011001122222222222", "El carácter '2' no es válido en base 2");
    }


}
    

