# macro-ob

## Abstract:

macro_ob is a symple program to demonstrate the dificulty of smart completions in rust macros.
this is done by allowing the inlining of rot13 in code.
this is only the start of what can be done with macros.

## example

    use macro-ob;
	    
    ob!(
        sa pbqrq() -> fgevat {
            "literals are not changed ".gb_fgevat()
        }
    )
    //main
    fn main {
        println!("{}", ob!(pbqrq()))
    }"# macro_ob" 
