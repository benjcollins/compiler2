use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{parser::ast::{self, Expr, Function, Statement}, scope::Scope};

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Unknown,
}

pub fn type_check_function<'a>(function: &Function<'a>) -> Result<Scope<'a>, ()> {
    let mut scope = Scope::new();
    for param in &function.params {
        for ident in param.idents {
            let ty = param.ty.map(|ty| convert_type(&ty)).unwrap_or(Type::Unknown);
            scope.insert(ident, Rc::new(RefCell::new(ty)));
        }
    }
    for stmt in &function.statements {
        match stmt {
            Statement::Declaration { decls, expr } => {
                for decl in decls {
                    for ident in &decl.idents {
                        let ty = decl.ty.map(|ty| convert_type(&ty)).unwrap_or(Type::Unknown);
                        scope.insert(ident, Rc::new(RefCell::new(ty)));
                    }
                    for ident in decls.iter().flat_map(|decl| decl.idents.iter()) {

                    }
                }
            }
            Statement::Assignment { idents, expr } => {
                let types = types_of_expr(expr, &scope);
                if types.len() != idents.len() {
                    panic!()
                }
                for (ident, ty) in idents.iter().zip(types) {
                    unify_types(&mut scope.get(ident).unwrap().borrow_mut(), &mut ty.borrow_mut())?;
                }
            }
            Statement::Return(expr) => {
                convert_type(function.ret)
            }
        }
    }
    Ok(scope)
}

fn assign_expr<'a>(expr: &Expr, scope: &Scope<'a>, idents: impl Iterator<Item = &'a str>) -> Result<(), ()> {
    let types = types_of_expr(expr, scope);
    let mut types_iter = types.iter();
    for (ident, ty) in idents.zip(types_iter) {
        unify_types(&mut scope.get(ident).unwrap().borrow_mut(), &mut ty.borrow_mut())?;
    }
    if !types_iter.is_empty() || !idents.
    Ok(())
}

fn types_of_expr<'a>(expr: &Expr, scope: &Scope<'a>) -> Vec<Rc<RefCell<Type>>> {
    match expr {
        Expr::Tuple(exprs) => exprs.iter().map(|expr| type_of_expr(expr, scope)).collect(),
        _ => vec![type_of_expr(expr, scope)],
    }
}

fn type_of_expr<'a>(expr: &Expr, scope: &Scope<'a>) -> Rc<RefCell<Type>> {
    match expr {
        Expr::IntLiteral(_) => Rc::new(RefCell::new(Type::Int)),
        Expr::Ident(ident) => Rc::clone(scope.get(ident).unwrap()),
        Expr::Tuple(_) => panic!(),
    }
}

fn unify_types(a: &mut Type, b: &mut Type) -> Result<(), ()> {
    match (a, b) {

        (_, Type::Unknown) => {
            *b = *a;
            Ok(())
        },

        (Type::Unknown, _) => {
            *a = *b;
            Ok(())
        }

        (Type::Int, Type::Int) => Ok(()),
        _ => Err(()),
    }
}

fn convert_type(ty: &ast::Type) -> Type {
    match ty.ident {
        "int" => Type::Int,
        _ => panic!()
    }
}