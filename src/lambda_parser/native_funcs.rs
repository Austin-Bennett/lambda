pub mod lambda_native {
    use std::io::{stdin, stdout};
    use std::process::{exit, id};
    use crossterm::{cursor, execute};
    use crossterm::cursor::MoveTo;
    use crossterm::terminal::{Clear, ClearType};
    use rand::random;
    use crate::lambda_parser::native_funcs::lambda_native;
    use crate::lambda_parser::{Env, ExprNode, FloatHelpers, Variable};
    use crate::lambda_parser::ExprNode::{Num};
    use crate::lambda_parser::Variable::Number;

    macro_rules! return_func {
        ($func: expr, $nodes: expr) => { return Ok(ExprNode::CallOperation { caller: Box::new(ExprNode::Ident($func.to_string())), args: $nodes } ) };
    }

//     let Some(pred) = {
        // let Some(pred_n) = inps.next() else { return_func!("if", nodes) };
    //     pred_n.solve_ref(varmap)?;
        // if let ExprNode::Ident(id) = pred_n {
        //      varmap.get(id)
        // } else if let ExprNode::Num(n) = pred_n {
        //      Some(&Variable::Number(*n))
        // } else {
        //      return_func!("if", nodes);
        // }
    // };

    macro_rules! lambda_native_func {
        ($id: ident($($arg: ident),*$(,)?) {$($body: tt)*}) => {
            #[allow(unused)]
            pub fn $id(varmap: &mut Env, mut nodes: Vec<ExprNode>) -> Result<ExprNode, String> {

                let mut inps = nodes.iter_mut();
                let strid = stringify!($id);
                $(
                    let Some($arg) = ({
                        let Some(n) = inps.next() else { return_func!(strid, nodes) };
                        n.solve_ref(varmap)?;
                        if let ExprNode::Ident(id) = n {
                            varmap.get_mut(id).cloned()
                        } else if let ExprNode::Num(n) = n {
                            Some(Variable::Number(*n))
                        } else {
                            return_func!(strid, nodes);
                        }
                    }) else { return_func!(strid, nodes) };
                )*

                return match { $($body)* } {
                    Some(n) => Ok(n),
                    None => return_func!(strid, nodes)
                };
            }
        };
    }






    pub fn init(varmap: &mut Env) {
        varmap.insert("quit", Variable::NativeFunction(&lambda_native::quit));
        varmap.insert("undecl", Variable::NativeFunction(&lambda_native::undecl));
        varmap.insert("clear", Variable::NativeFunction(&lambda_native::clear));
        varmap.insert("if", Variable::NativeFunction(&lambda_native::__if));
        varmap.insert("rand", Variable::NativeFunction(&lambda_native::rand));
        varmap.insert("sqrt", Variable::NativeFunction(&lambda_native::sqrt));
        varmap.insert("ln", Variable::NativeFunction(&lambda_native::ln));
        varmap.insert("log", Variable::NativeFunction(&lambda_native::log));
    }


    lambda_native_func!(rand() {
        Some(ExprNode::Num(random::<f64>()))
    });


    lambda_native_func!(clear() {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        Some(ExprNode::Void)
    });

    pub fn quit(varmap: &mut Env, mut nodes: Vec<ExprNode>) -> Result<ExprNode, String> {
        let Some(code) = nodes.first_mut() else { exit(0) };
        let Some(code) = code.solve_ref(varmap)?.to_number(varmap) else { exit(0) };
        exit(code as i32);
    }

    pub fn undecl(varmap: &mut Env, mut nodes: Vec<ExprNode>) -> Result<ExprNode, String> {
        if let Some(ExprNode::Ident(ident)) = nodes.first_mut() {
            varmap.undeclare(ident);
            Ok(ExprNode::Void)
        } else {
            return_func!("undecl", nodes)
        }
    }

    pub fn __if(varmap: &mut Env, mut nodes: Vec<ExprNode>) -> Result<ExprNode, String> {
        //first value is the predicate, second and third are the true/false expressions
        let mut inps = nodes.iter_mut();


        let Some(pred) = ({
            let Some(pred_n) = inps.next() else { return_func!("if", nodes) };
            pred_n.solve_ref(varmap)?.to_number(varmap)
        }) else { return_func!("if", nodes) };



        let Some(true_n) = inps.next() else { return_func!("if", nodes) };
        let Some(false_n) = inps.next() else { return_func!("if", nodes) };




        if !pred.equates(0.0) {
            Ok(std::mem::replace(true_n, Num(0.0)).solve(varmap)?)
        } else {
            Ok(std::mem::replace(false_n, Num(0.0)).solve(varmap)?)
        }
    }

    lambda_native_func!(sqrt(x) {

        if let Number(x) = x {
            Some(ExprNode::Num(x.sqrt()))
        } else {
            None
        }
    });

    lambda_native_func!(ln(x) {

        if let Number(x) = x {
            Some(ExprNode::Num(x.ln()))
        } else {
            None
        }
    });

    lambda_native_func!(log(x, b) {

        if let Number(x) = x && let Number(b) = b {
            Some(ExprNode::Num(x.log(b)))
        } else {
            None
        }
    });


}


