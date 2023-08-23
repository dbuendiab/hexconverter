//! # Estructura Numero
//! 
//! Permite guardar una cantidad en una base dada entre 2 y 35
//! y convertirla a otra base, accediendo a su representación
//! su valor y la base correspondiente
//! 
//! Implementa además el trait Display, que equivale al repr()
//! de Python y permite rellenar el {} en los println!()

// Rutas del trait y de su raíz (self = str::fmt)
use std::fmt::{Display, self};

// El módulo dígitos implementa la correspondencia num <-> char
// para todos los dígitos posibles de la aplicación
use crate::digitos;

/// # Estructura Numero
/// 
/// Permite guardar números naturales en cualquier base numérica entre 2 y 35

pub struct Numero {
    /// Cadena alfanumérica que representa el número (en su base original)
    text_value: String, 

    /// Base numérica del número - entre 2 y 32
    base: usize,

    /// Valor numérico en base 10
    val10: usize,
}

// Implementación de las funciones de la estructura
impl Numero {

    /// Comprobar que la base propuesta está dentro de los márgenes 2 <= base <= 35
    fn validar_base(base_number: usize) -> bool { base_number >= 2 && base_number <= 35 }

    /// Devuelve un Result con el Número, si todo fue bien, o una cadena de error si no
    pub fn new(text_value: String, base_number: usize) -> Result<Self, String> {

        // Comprobar validez base (y conversión a usize)
        let base_number: usize = if Numero::validar_base(base_number) {
            base_number
        } else {
            return Err("Base fuera de límites".to_owned());
        };

        // Pasar la cadena a minúsculas
        let text_value: String = text_value.to_ascii_lowercase();

        // Guardar el rango de caracteres válidos
        // (de momento no, usaré directamente DIGITOS en digit.rs)

        // Comprobar la validez de los caracteres (y quizás pasar a [Chars])
        let text_value_inverted:std::iter::Rev<std::str::Chars<'_>> = text_value.chars().rev();

        // De paso, puedo calcular el valor decimal en el bucle
        let mut valor_10 = 0_usize;
        let mut potencia = 1_usize;

        for letra in text_value_inverted {

            match digitos::get_num(letra) {
                Some(numero) => {
                    // El dígito puede ser válido pero no para la base actual
                    if numero > (base_number - 1) {
                        return Err(format!("El carácter '{letra}' no es válido en base {base_number}"))
                    }

                    // Cálculo del valor del dígito y añadido al total
                    valor_10 += numero * potencia;  // Valor del dígito actual
                    potencia *= base_number;        // Incrementar la potencia de la base
                },
                None => {
                    return Err(format!("El carácter '{letra}' en '{text_value}' no es válido"));
                },
            };

        }

        Ok(Self {
            text_value: text_value,
            base: base_number,
            val10: valor_10,
        })
    }


    /// Devuelve el valor decimal del número
    pub fn value(&self) -> usize { self.val10 }


    /// Devuelve la cadena alfanumérica que representa al número
    pub fn face(&self) -> &String { &self.text_value }


    // Genera un nuevo Numero en una base distinta (encapsulado en Result)
    pub fn to_base(&self, base_dest: usize) -> Result<Self, String> {

        if Numero::validar_base(base_dest) == false {
            return Err(format!("La base numérica {base_dest} no es válida"));
        }

        let mut digitos: Vec<usize> = Vec::new();

        let valor = self.val10;

        // El 0 y el 1 son iguales en todas las bases
        if valor <= 1 {
            return Ok( Numero {text_value: valor.to_string(), base: base_dest, val10: valor,} );
        }

        if base_dest == 10 {
            return Ok( Numero {text_value: valor.to_string(), base: base_dest, val10: valor,} );
        }

        // Caso base_dest != 10
        else {
            let mut dividendo = valor;
            let divisor = base_dest;

            loop {
                let cociente = dividendo / divisor;
                let resto = dividendo % divisor;

                digitos.push(resto);
                if cociente < base_dest {
                    digitos.push(cociente);
                    break;
                }
                dividendo = cociente;
            }

            // Invertir el vector 
            let text_value = digitos
                .iter()
                .rev()
                .map(|n: &usize| digitos::get_char(*n).unwrap_or('?'))
                .collect();

            // Combinar en un String

            // Componer el número y devolver
            Ok(
                Numero {
                    text_value: text_value, 
                    base: base_dest,
                    val10: valor,
                }
            )
        }


        // Convertir los restos de las divisiones en dígitos

    }

}

// Implementación del trait Display (para las visualizaciones)
impl Display for Numero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.text_value, self.base)
    }
}