# macro-ob

## Abstract:

macro-ob is a symple program to demonstrate the dificulty of smart completions in rust macros.
this is done by allowing the inlining of rot13 in code.
this is only the start of what can be done with macros.

## example

### Cargo.toml

    macro-ob = "0.1.3"

### main.rs

    use macro_ob;   
    rot!(
        sa pbqrq() -> Fgevat {
            "literals are not changed for tecnical resions".gb_fgevat()
        }
    )
    fn main {
        println!("{}", rot!(pbqrq()))
    }
