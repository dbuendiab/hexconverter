//! # Aplicación hexconverter
//! 
//! Para poder usar otros ficheros .rs en este main.rs, mod incluye el código del fichero entero
//! digit contiene el módulo digit, con las funciones get_char() y get_num()
//! numero contiene la estructura Numero y su implementación
//! A pesar de ser cosas distintas, el tratamiento es homogéneo.
mod digit;
mod numero;

// Crates del sistema
use std::env;           // Crate para tratamiento de argumentos
use std::error::Error;  // Crate para el Error del main()

// Asigna una ruta completa a un nombre
// En este caso, digitos es un módulo, y Numero es la estructura Numero
use digit::digitos;
use numero::Numero;

// Mensaje de información del programa
const INFO: &str = 
r#"
    HexConverter: utilidad de conversión de bases numéricas

    Uso: hexconverter <numero> <base1> <base2>");
"#;


#[doc(hidden)]
fn main() -> Result<(), Box<dyn Error>> {

    // Tratamiento del vector de argumentos: número de argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("{}", INFO);
        std::process::exit(1);
    }
    // Recogida de los parámetros. Interesante el Result del parse(), resuelto vía expect()
    let numero = &args[1];
    let base1: usize = args[2].parse().expect("El segundo argumento debe ser un número entero");
    let base2: usize = args[3].parse().expect("El tercer argumento debe ser un número entero");

    // Aquí usamos las funciones del crate numero::Numero
    // Interesante el uso de (?) para hacer unwrap() pero sin panic!
    // (en lugar de ello, se devuelve el Ok() a la variable, y si hay Err() sale directamente del programa)
    // Para que esto funcione, la función main() debe devolver Result<(), Box <dyn Error>>
    // Otra opción es except(), pero en este caso, aunque devuelve el mensaje, se provoca un panic!()
    let n = Numero::new(String::from(numero), base1)?;
    let n2 = n.to_base(base2)?;

    // Presentación de los resultados
    println!("Conversión {} -> {} -> {}", n, n.value(), n2);

    // Es obligado devolver esto para poder usar (?)
    Ok(())
}
