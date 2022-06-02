use crate::env::Env;
use crate::val::Val;
use crate::{stmt::Stmt, utils};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::skip_whitespace(s);

        let mut s = s;
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Stmt::new(s) {
            stmts.push(stmt);
            let (new_s, _) = utils::skip_whitespace(new_s);
            s = new_s
        }

        let (s, _) = utils::skip_whitespace(s);
        let s = utils::tag("}", s)?;
        Ok((s, Self { stmts }))
    }

    pub fn eval(&self, _: &Env) -> Result<Val, String> {
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }

        let mut env = Env::default();
        let stmts_except_last = &self.stmts[..self.stmts.len() - 1];
        for stmt in stmts_except_last {
            stmt.eval(&mut env)?;
        }

        self.stmts.last().unwrap().eval(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;
    use crate::expr::Expr;
    use crate::expr::Number;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })))
    }

    #[test]
    fn parse_empty_block_with_spaces() {
        assert_eq!(Block::new("{    }"), Ok(("", Block { stmts: Vec::new() })))
    }

    #[test]
    fn parse_block_with_one_statement() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))]
                }
            ))
        )
    }

    #[test]
    fn parse_block_with_multiple_statement() {
        assert_eq!(
            Block::new(
                "{ 
                    let a = 10
                    let b = a
                    b
                }"
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10))
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string(),
                            }),
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string(),
                        })),
                    ]
                }
            ))
        )
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: Vec::new() }.eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(10)))]
            }
            .eval(&Env::default()),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn parse_block_with_binding_def_and_usage() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "two".to_string(),
                        val: Expr::Number(Number(2))
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name: "two".to_string()
                    }))
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Number(2))
        )
    }

    #[test]
    fn eval_block_with_multiple_binding_defs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "jump".to_string(),
                        val: Expr::Number(Number(22))
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "cut".to_string(),
                        val: Expr::Number(Number(200))
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "pre".to_string(),
                        val: Expr::Number(Number(32))
                    })
                ]
            }
            .eval(&mut Env::default()),
            Ok(Val::Unit)
        )
    }
}
