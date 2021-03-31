use std::{cell::RefCell, iter::Once, rc::Rc, slice::Iter};

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
                let tys = expr.map(|expr| types_of_expr(&expr, &scope));
                for decl in decls {
                    for ident in &decl.idents {
                        let ty = decl.ty.map(|ty| convert_type(&ty)).unwrap_or(Type::Unknown);
                        unify_types(ty, b)
                        scope.insert(ident, Rc::new(RefCell::new(ty)));
                    }
                }
                match expr {
                    Some(expr) => {
                        
                    }
                    None => {},
                }
            }
            Statement::Assignment { idents, expr } => {
                let types = types_of_expr(expr, &scope);
                if types.len() != idents.len() {
                    panic!()
                }
                for (ident, ty) in idents.iter().zip(types) {
                    unify_types(&mut scope.get(ident).unwrap().as_ref().borrow_mut(), &mut ty.as_ref().borrow_mut())?;
                }
            }
            Statement::Return(expr) => {
                let tys = types_of_expr(expr, &scope);
                for (ty, ret) in tys.iter().zip(function.rets) {
                    unify_types(&mut ty.as_ref().borrow_mut(), &mut convert_type(&ret))?;
                }
            }
        }
    }
    Ok(scope)
}

enum TypeIter<'a> {
    TupleIter(Iter<Expr<'a>>),
    Once(Once<Expr<'a>>),
}

impl<'a> Iterator for TypeIter<'a> {
    type Item = Rc<RefCell<Type>>;

    fn next(&mut self) -> Option<Self::Item> {
        let expr = match self {
            TypeIter::TupleIter(iter) => iter.next(),
            TypeIter::Once(once) => once.next(),
        }?;
        type_of_expr(expr, scope);
    }
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