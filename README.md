# hexconverter

Aplicación de línea de comandos en Rust que convierte números entre bases numéricas (2–35).

## Uso

```
hexconverter <numero> <base1> <base2>
```

**Ejemplo:**
```
hexconverter 1001010 2 16
Conversión 1001010:2 -> 74 -> 4a:16
```

La salida tiene el formato: `<numero>:<base1> -> <valor decimal> -> <resultado>:<base2>`

**Parámetros:**
- `numero`: el número a convertir (dígitos `0-9`, letras `a-z` o `A-Z`)
- `base1`: base de origen (2–35)
- `base2`: base de destino (2–35)

Las mayúsculas se convierten automáticamente a minúsculas.

> **Nota:** La base 35 usa los dígitos `0-9` y `a-y` (35 símbolos). El carácter `z` (valor 35) **no es válido** en ninguna base soportada.

---

## Estructura del proyecto

```
hexconverter/
├── Cargo.toml
├── src/
│   ├── lib.rs                  # Exportaciones públicas y documentación
│   ├── main.rs                 # Punto de entrada CLI
│   ├── digit.rs                # Mapeo bidireccional carácter ↔ dígito
│   ├── numero.rs               # Struct Numero y lógica de conversión
│   └── bin/
│       └── test_integracion.rs # Tests de integración (binario separado)
├── tests/
│   └── test_num_digit.rs       # Tests unitarios externos
└── tests.json                  # Casos de prueba para integración
```

**Dependencias de módulos:**
```
main.rs → numero.rs → digit.rs
```

`serde` y `serde_json` se usan solo en dev (tests de integración).

---

## Algoritmo de conversión

**Base → Decimal (`Numero::new`):**
- Recorre la cadena de entrada al revés
- Para cada carácter obtiene su valor numérico con `get_num()`
- Acumula: `valor += digito * base^posicion`

**Decimal → Base destino (`Numero::to_base`):**
- Casos especiales: 0, 1 y base 10 (retorno directo)
- División repetida: acumula restos, los invierte
- Mapea cada resto a carácter con `get_char()`

---

## Tests

```
cargo test
```

**22 tests en total, todos pasan:**

| Suite | Tests | Descripción |
|---|---|---|
| `digit.rs` (inline) | 4 | Mapeo carácter ↔ índice, incluyendo casos `None` |
| `test_num_digit.rs` | 9 | Construcción de `Numero`, validación de base, conversiones |
| `test_integracion.rs` | 2 | Ejecución del binario completo vía `Command` |
| Doc-tests | 3 | Ejemplos en comentarios `///` de `digit.rs` y `numero.rs` |

Los tests de integración cargan casos desde `tests.json`. Si el fichero no existe, usan una constante de fallback.

---

## Bugs conocidos

### 1. Overflow silencioso al convertir números grandes
**Archivo:** `numero.rs:94`
**Severidad:** Media

La acumulación `valor_10 += numero * potencia` puede desbordar `usize` sin producir error. En sistemas de 64 bits, `usize::MAX` es ~18.4 quintillones, pero no se valida el desbordamiento.

```rust
// Línea problemática:
valor_10 += numero * potencia;
// Si desborda, wrappea silenciosamente en debug builds no configurados con overflow-checks
```

**Workaround futuro:** usar `checked_add` / `checked_mul`, o migrar a `u128` / `BigInt`.

---

### 2. `unwrap_or('?')` enmascara fallos internos
**Archivo:** `numero.rs:162`
**Severidad:** Baja (nunca debería ocurrir en práctica)

```rust
.map(|n: &usize| digitos::get_char(*n).unwrap_or('?'))
```

Si `get_char` devuelve `None` para un resto válido (0–34), el resultado contendrá `'?'` sin aviso. Debería ser `.expect("resto fuera de rango")`.

---

### 3. `.expect()` en `main.rs` hace panic con bases no numéricas
**Archivo:** `main.rs:40-41`
**Severidad:** Baja (UX)

```rust
let base1: usize = args[2].parse().expect("El segundo argumento debe ser un número entero");
```

Si el usuario pasa letras como base (e.g. `hexconverter 35 abc 2`), el programa hace panic en lugar de mostrar un error limpio. Debería usar `.map_err(...)?`.

---

## Warnings de Clippy (estilo, no funcionales)

| Archivo | Línea | Problema | Fix |
|---|---|---|---|
| `numero.rs` | ~104 | `text_value: text_value` | → `text_value` |
| `numero.rs` | ~168 | ídem | → campo abreviado |
| `numero.rs` | ~46 | `>= 2 && <= 35` | → `(2..=35).contains(&base)` |
| `numero.rs` | ~134 | `return Ok(...)` explícito | → expresión implícita |
| `numero.rs` | ~120 | `== false` | → `!Numero::validar_base(...)` |

---

## Gaps en la cobertura de tests

Casos no cubiertos actualmente:

- **Simetría de conversión:** verificar que `A → base_B → base_A == A`
- **Entrada vacía:** comportamiento de `Numero::new("", base)`
- **Cadena con espacios o caracteres especiales**
- **CLI con bases inválidas** (letras, negativos, cero, uno): actualmente hace panic
- **Carácter `'y'` en base 35** (el dígito máximo válido, valor 34)
- **Números cerca de `usize::MAX`** para verificar el overflow

---

## Features pendientes / ideas

| Feature | Descripción |
|---|---|
| Modo silencioso | `--quiet`: imprime solo el resultado, sin decoración |
| Cadena de bases | `35 2 8 16`: conversiones encadenadas en un solo comando |
| Verificación de redondeo | Comprobar que `N → B1 → B2 → B1 == N` |
| Soporte BigInt | Números de precisión arbitraria (crate `num-bigint`) |
| Modo interactivo | Bucle REPL para múltiples conversiones sin relanzar el proceso |
| Opciones de formato | Prefijos `0x`, `0b`, `0o` en la salida |

---

## Ejecución

```bash
# Compilar y ejecutar
cargo run --bin hexconverter 35 10 2

# Solo compilar
cargo build

# Build optimizado
cargo build --release
# Binario en: target/release/hexconverter.exe

# Tests
cargo test

# Linter
cargo clippy

# Documentación
cargo doc --open
```
