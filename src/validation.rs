use crate::ast::*;
use crate::error::CompilerError;
use std::collections::HashMap;

pub fn validate(program: &Program) -> Result<(), CompilerError> {
    conflicting_function_definitions(program)?;
    undeclared_function_call(program)?;
    Ok(())
}

fn validate_all_expr(
    statement: &Statement,
    f: &dyn Fn(&Expression) -> Result<(), CompilerError>,
) -> Result<(), CompilerError> {
    match statement {
        Statement::Declaration(_, _, expr) => expr.as_ref().map_or(Ok(()), f),
        Statement::Return(expr) => f(expr),
        Statement::Expression(expr) => expr.as_ref().map_or(Ok(()), f),
        Statement::If(expr, stm1, stm2) => {
            f(expr)?;
            validate_all_expr(stm1, f)?;
            stm2.as_ref().map_or(Ok(()), |s| validate_all_expr(s, f))
        }
        Statement::Compound(stms) => {
            for s in stms.iter() {
                validate_all_expr(s, f)?;
            }
            Ok(())
        }
        Statement::For(expr1, expr2, expr3, stm) => {
            expr1.as_ref().map_or(Ok(()), f)?;
            f(expr2)?;
            expr3.as_ref().map_or(Ok(()), f)?;
            validate_all_expr(stm, f)
        }
        Statement::ForDecl(stm1, expr1, expr2, stm2) => {
            f(expr1)?;
            expr2.as_ref().map_or(Ok(()), f)?;
            validate_all_expr(stm1, f)?;
            validate_all_expr(stm2, f)
        }
        Statement::While(expr, stm) => {
            validate_all_expr(stm, f)?;
            f(expr)
        }
        Statement::Do(stm, expr) => {
            validate_all_expr(stm, f)?;
            f(expr)
        }
        Statement::Break | Statement::Continue => Ok(()),
    }
}

fn undeclared_function_call(program: &Program) -> Result<(), CompilerError> {
    let mut fun_map: HashMap<&Identifier, Vec<Type>> = HashMap::new();
    for Function {
        name,
        args,
        statements,
    } in program.funs.iter()
    {
        let args_types: Vec<Type> = args.iter().map(|a| a.0.clone()).collect();
        fun_map.insert(name, args_types);

        for stm in statements.as_ref().unwrap_or(&Vec::new()).iter() {
            validate_all_expr(stm, &|e| {
                if let Expression::FunCall(id, args) = e {
                    // TODO: typecheck args
                    if !(fun_map.contains_key(&id) && fun_map.get(&id).unwrap().len() == args.len())
                    {
                        return Err(CompilerError::Validation(format!(
                            "Undeclared function {}",
                            id
                        )));
                    }
                }
                Ok(())
            })?;
        }
    }
    Ok(())
}

fn conflicting_function_definitions(program: &Program) -> Result<(), CompilerError> {
    let mut fun_map: HashMap<&Identifier, Vec<Type>> = HashMap::new();
    for Function {
        name,
        args,
        statements: _,
    } in program.funs.iter()
    {
        let args_types: Vec<Type> = args.iter().map(|a| a.0.clone()).collect();

        if fun_map.contains_key(name) && !fun_map.get(name).unwrap().iter().eq(args_types.iter()) {
            return Err(CompilerError::Validation(format!(
                "Conflicting definitions for function {}",
                name
            )));
        }

        fun_map.insert(name, args_types);
    }
    Ok(())
}
