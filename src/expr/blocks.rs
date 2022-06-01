use crate::{utils, stmt::Stmt};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::skip_whitespace(s);

        let (s, stmts) = if let Ok((s, stmt)) = Stmt::new(s) {
            (s, vec![stmt])
        } else {
            (s, Vec::new())
        };

        let (s, _) = utils::skip_whitespace(s);
        let s = utils::tag("}", s)?;
        Ok((s, Self { stmts }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
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
                Block { stmts: vec![Stmt::Expr(Expr::Number(Number(5)))] }
            )))
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
                Block { stmts: 
                    vec![
                        Stmt::BindingDef(BindingDef{ name: "a".to_string(), val: Expr::Number(Number(10))}),
                        Stmt::Expr(Expr::Number(Number(5))),
                        Stmt::Expr(Expr::Number(Number(5))),
                    ]
                }
            )))
    }
    
}
