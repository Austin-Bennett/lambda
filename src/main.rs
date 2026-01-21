use crate::lambda_parser::native_funcs::lambda_native;
use crate::lambda_parser::{Env, ExprNode};
use std::collections::HashMap;
use std::io::Write;

mod lambda_parser;
mod macros;


fn main() -> std::io::Result<()> {

    print!("> ");
    std::io::stdout().flush()?;
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp)?;
    inp = inp.trim().to_string();

    let mut varmap = Env{ scopes: vec![HashMap::new()] };

    lambda_native::init(&mut varmap);

    while inp != "QUIT" {
        match ExprNode::from_str(&inp) {
            Ok(n) => {
                match n.solve(&mut varmap) {
                    Ok(n ) => {
                        if let ExprNode::Void = n {

                        } else {
                            println!("{:?}", n)
                        }
                    }
                    Err(s) => println!("Error: {}", s)
                }
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }

        print!("> ");
        std::io::stdout().flush()?;
        inp.clear();
        std::io::stdin().read_line(&mut inp)?;
        inp = inp.trim().to_string();
    }

    Ok(())
}
