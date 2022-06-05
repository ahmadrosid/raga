pub mod binding_def;
mod env;
pub mod expr;
pub mod stmt;
mod utils;
pub mod val;

pub use val::Val;
pub use env::Env;

#[derive(Debug)]
pub struct Parse(stmt::Stmt);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        dbg!(s);
        Err("input was not consumed fully by parser".to_string())
    }
}