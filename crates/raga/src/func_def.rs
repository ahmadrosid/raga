use crate::stmt::Stmt;
use crate::{Env, utils};

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Stmt>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;
        let (s, _) = utils::extract_whitespace(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::skip_whitespace(s);

        let (s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(s, ident)| (s, ident.to_string())),
            s
        )?;

        let s = utils::tag("=>", s)?;
        let (s, _) = utils::skip_whitespace(s);

        let (s, body) = Stmt::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params,
                body: Box::new(body),
            }
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_func(self.name.clone(), self.params.clone(), *self.body.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Env, Val};
    use super::*;
    use crate::expr::{BindingUsage, Block, Expr, Number, Op};

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn newEmpty => {}"),
            Ok((
                "",
                FuncDef {
                    name: "newEmpty".to_string(),
                    params: vec![],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: vec![] }))),
                }
            ))
        )
    }

    #[test]
    fn parse_func_def_with_one_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn newEmpty x => {}"),
            Ok((
                "",
                FuncDef {
                    name: "newEmpty".to_string(),
                    params: vec!["x".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: vec![] }))),
                }
            ))
        )
    }

    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn newEmpty x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "newEmpty".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_string()
                        })),
                        op: Op::Add
                    }))
                }
            ))
        )
    }

    #[test]
    fn eval_func_def() {
        assert_eq!(
            Stmt::FuncDef(FuncDef {
                name: "return_one".to_string(),
                params: vec![],
                body: Box::new(Stmt::Expr(Expr::Number(Number(1))))
            }).eval(&mut Env::default()),
            Ok(Val::Unit)
        )
    }
}