use crate::env::Env;
use crate::expr::Expr;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespace(s)?;

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::skip_whitespace(s);

        let s = utils::tag("=", s)?;

        let (s, _) = utils::skip_whitespace(s);
        let (s, val) = Expr::new(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_binding(self.name.clone(), self.val.eval(env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Box::new(Expr::Number(Number(10))),
                        rhs: Box::new(Expr::Number(Number(2))),
                        op: Op::Div
                    }
                }
            ))
        )
    }

    #[test]
    fn can_not_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaa= 10 / 2"),
            Err("expected a space".to_string())
        )
    }

    #[test]
    fn parse_block_with_no_space() {
        assert_eq!(
            BindingDef::new("let aa=20"),
            Ok((
                "",
                BindingDef {
                    name: "aa".to_string(),
                    val: Expr::Number(Number(20))
                }
            ))
        )
    }
}
