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

    #[test]
    fn hexconverter_run_conversiones() {
        let tests = [
            ["1000", "2", "10", "Conversión 1000:2 -> 8 -> 8:10\n"],
            ["1ba0", "2", "10", "Error: \"El carácter 'b' no es válido en base 2\"\n"],
        ];

        for row in tests.iter() {
            primitiva(row);
        }
        }
}
