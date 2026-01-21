pub mod lambda_native {
    use crate::lambda_parser::native_funcs::lambda_native;
    use crate::lambda_parser::{Env, ExprNode, Variable};
    use crate::lambda_parser::ExprNode::Void;

    macro_rules! return_func {
        ($func: literal, $nodes: expr) => { return Ok(ExprNode::CallOperation { caller: Box::new(ExprNode::Ident($func.to_string())), args: $nodes } ) };
    }


    pub fn init(varmap: &mut Env) {
        varmap.insert("sqrt", Variable::NativeFunction(&lambda_native::sqrt));
        varmap.insert("if", Variable::NativeFunction(&lambda_native::__if));
    }



    pub fn __if(varmap: &mut Env, mut nodes: Vec<ExprNode>) -> Result<ExprNode, String> {
        //first value is the predicate, second and third are the true/false expressions
        let mut inps = nodes.iter_mut();

        let Some(pred_n) = inps.next() else { return_func!("if", nodes) };
        let Some(true_n) = inps.next() else { return_func!("if", nodes) };
        let Some(false_n) = inps.next() else { return_func!("if", nodes) };

        let Some(pred) = pred_n.solve(varmap)?.to_number(varmap) else { return_func!("if", nodes) };

        Ok(Void)
    }

    pub fn sqrt(varmap: &mut Env, mut nodes: Vec<ExprNode>) -> Result<ExprNode, String> {
        if let Some(node) = nodes.first_mut() && let Some(n) = {
            let old = std::mem::replace(node, ExprNode::Num(0.0));
            *node = old.solve(varmap)?;
            node.to_number(varmap)
        } {

            return Ok(ExprNode::Num(n.sqrt()));
        } else {
            return Ok(ExprNode::CallOperation { caller: Box::new(ExprNode::Ident("sqrt".to_string())), args: nodes })
        }
    }


}


