use crate::{utils, env::Env, val::Val};

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    pub name: String
}

impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;
        Ok((
            s,
            Self {
                name: name.to_string()
            }
        ))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding_value(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abs"),
            Ok((
                "",
                BindingUsage {
                    name: "abs".to_string()
                }
            ))
        )
    }
}