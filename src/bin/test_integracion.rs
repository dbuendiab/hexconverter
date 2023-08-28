fn main() {
    println!("Hola mundo");
}

#[cfg(test)]

mod test {
    use std::process::Command;

    const PROGRAM: &str = "hexconverter.exe"; // Lo va a buscar directamente a target\debug (!)

    #[test]
    fn hexconverter_run_ok() {
        let output = Command::new(PROGRAM).output().unwrap();
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn hexconverter_run_conversiones() {

        let tests:Vec<Vec<String>> = read_json_tests(FICHERO_JSON);

        // Convertir el Vec<Vec<String>> en un Vec<Vec<&str>>
        // Se hace un map() sobre el vector exterior y dentro de ese map()
        // se hace otro map() interior recorriendo las Strings del vector
        // para convertirlas en &str
        let vec_of_str_refs: Vec<Vec<&str>> = tests
                                                .iter()
                                                .map(|s| 
                                                    s.iter()
                                                    .map(|s2| s2.as_str())
                                                    .collect()
                                                )
                                                .collect();

        for row in vec_of_str_refs.iter() {
            primitiva(row);
        }
    }

    //use serde::Deserialize;
    use std::fs::File;
    use std::io::Read;

    const FICHERO_JSON: &str = "tests.json";
    const DEFAULT_TESTS: &str= r#"[
            ["1000", "2", "10", "Conversión 1000:2 -> 8 -> 8:10\n"],
            ["1ba0", "2", "10", "Error: \"El carácter 'b' no es válido en base 2\"\n"]
        ]"#;
        
/* 
    // Mi versión. La de ChatGPT (a partir de la mía) es más idiomática, por eso es la que dejo 
    
    fn read_json_tests(fichero: &str) -> Vec<Vec<String>> {

        // Creamos un valor por defecto para los tests (el que usábamos originalmente)
        // Esto da un error irrecuperable si la cadena está mal escrita en el código fuente
        let default = serde_json::from_str(DEFAULT_TESTS).unwrap();

        // Tratamos de recuperar otros tests desde el fichero 
        let file = File::open(fichero);
        let mut content = String::new();

        // Si hay fichero, tratarlo. Si no, devolver el default
        match file {
            Ok(mut the_file) => {

                // Leer el contenido del fichero
                match the_file.read_to_string(&mut content) {

                    // Si la lectura es correcta, devolver el contenido
                    Ok(_) => { 
                        let json_tests/*: Result<Vec<Vec<String>>, serde_json::Error>*/ = serde_json::from_str(&content);
                        match json_tests {
                            Ok(jtests) => { return jtests; },
                            Err(_) => { return default; }
                        }
                     },
                    // Si no, devolver el default
                    Err(_) => { return default; }
                }
            },
            // Hubo error al abrir el fichero
            Err(_) =>{ return default; }
        }
    }
 */

    // Solución simplificada de ChatGPT a partir de la original. El caso es que funciona
    // La construcción diferencial es la línea if let Ok(mut_the_file)..., que es un match simplificado
    // que solo trata el caso Ok() del Result(). La siguiente linea también es especial, ya que el Result()
    // se convierte en Option via .ok()
    // Otra cosa interesante es el .unwrap_or_else() del final

    fn read_json_tests(fichero: &str) -> Vec<Vec<String>> {

        // Leer JSON de texto (hardcoded en este mismo módulo)
        let default_tests: Vec<Vec<String>> = serde_json::from_str(DEFAULT_TESTS).unwrap();

        // Tratar de leer el contenido del fichero (si no hay problemas de fichero o contenido)
        let mut content = String::new();
        if let Ok(mut the_file) = File::open(fichero) {
            the_file.read_to_string(&mut content).ok();
        }

        // Tratar de convertir el contenido JSON del fichero a formato tests, o usar el valor por defecto
        serde_json::from_str(&content).unwrap_or_else(|_| default_tests)
    }

    fn primitiva(argumentos_4: &[&str]) -> () {
        //numero: &str, base1: &str, base2: &str, out: &str) {

        let argumentos: Vec<String> = (*argumentos_4)
            .iter()
            .map(|s| s.to_string())
            .collect();

        match argumentos.as_slice() {
            [numero, base1, base2, texto_salida] => {
                let output = Command::new(PROGRAM)
                    .args([numero, base1, base2])
                    .output()
                    .unwrap();

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                if stdout != "" {
                    assert_eq!(stdout, *texto_salida);
                }
                if stderr != "" {
                    assert_eq!(stderr, *texto_salida);
                }
            }
            _ => {
                println!("No se encontraron cuatro elementos");
            }
        }
    }


}
