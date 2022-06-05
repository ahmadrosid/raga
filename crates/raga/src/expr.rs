pub mod binding_usage;
pub mod blocks;

use crate::env::Env;
use crate::{utils, val::Val};

pub(crate) use binding_usage::BindingUsage;
pub(crate) use blocks::Block;

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation {
        lhs: Box<Self>,
        rhs: Box<Self>,
        op: Op
    },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_non_operation(s))
    }

    pub fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub fn new_non_operation(s: &str) -> Result<(&str, Self), String> {
        Self::new_number(s)
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    pub fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Self::new_non_operation(s)?;
        let (s, _) = utils::skip_whitespace(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::skip_whitespace(s);

        let (s, rhs) = Self::new_non_operation(s)?;

        Ok((s, Self::Operation {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op
        }))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Operation { lhs, rhs, op } => {
                let lhs = lhs.eval(env)?;
                let rhs = rhs.eval(env)?;

                let (lhs, rhs) = match (lhs, rhs) {
                    (Val::Number(lhs), Val::Number(rhs)) => (lhs, rhs),
                    _ => return Err("cannot evaluate operation whose left-hand side and right-hand side are not both numbers".to_string())
                };

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Ok(Val::Number(result))
            }
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::env::Env;
    use crate::expr::binding_usage::BindingUsage;
    use crate::stmt::Stmt;

    #[test]
    fn parse_numbner() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("123"), Ok(("", Expr::Number(Number(123)))));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs:  Box::new(Expr::Number(Number(2))),
                    op: Op::Add,
                }
            ))
        );
    }

    #[test]
    fn parse_expr_white_whitespace() {
        assert_eq!(
            Expr::new("1 + 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs:  Box::new(Expr::Number(Number(1))),
                    rhs:  Box::new(Expr::Number(Number(2))),
                    op: Op::Add,
                }
            ))
        );
    }

    #[test]
    fn eval_non_number_operation() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Block(Block{ stmts: vec![] })),
                op: Op::Add
            }.eval(&Env::default()),
            Err("cannot evaluate operation whose left-hand side and right-hand side are not both numbers".to_string())
        )
    }

    #[test]
    fn eval_add() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation {
                lhs:  Box::new(Expr::Number(Number(1))),
                rhs:  Box::new(Expr::Number(Number(2))),
                op: Op::Add,
            }.eval(&env),
            Ok(Val::Number(3))
        )
    }

    #[test]
    fn eval_sub() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation {
                lhs:  Box::new(Expr::Number(Number(1))),
                rhs:  Box::new(Expr::Number(Number(2))),
                op: Op::Sub,
            }
            .eval(&env),
            Ok(Val::Number(-1))
        )
    }

    #[test]
    fn eval_mul() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation {
                lhs:  Box::new(Expr::Number(Number(10))),
                rhs:  Box::new(Expr::Number(Number(2))),
                op: Op::Mul,
            }
            .eval(&env),
            Ok(Val::Number(20))
        )
    }

    #[test]
    fn eval_div() {
        let env = Env::default();
        assert_eq!(
            Expr::Operation {
                lhs:  Box::new(Expr::Number(Number(10))),
                rhs:  Box::new(Expr::Number(Number(2))),
                op: Op::Div,
            }
            .eval(&env),
            Ok(Val::Number(5))
        )
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string()
                })
            ))
        )
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::new("{ 200 }"),
            Ok((
                "",
                Expr::Block(Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(200)))]
                })
            ))
        )
    }

    #[test]
    fn parse_block_with_multiple_statements() {
        assert_eq!(
            Expr::new(
                "{
                    let a = 10
                    let b = a
                    b
                }"
            ),
            Ok((
                "",
                Expr::Block(Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10)),
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
                })
            ))
        )
    }

    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.store_binding("ten".to_string(), Val::Number(10));

        assert_eq!(
            Expr::BindingUsage(BindingUsage {
                name: "ten".to_string()
            })
            .eval(&env),
            Ok(Val::Number(10))
        )
    }
}
