use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    bindings: HashMap<String, Val>,
}

impl Env {
    pub fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.bindings.get(name)
            .cloned()
            .ok_or_else(|| format!("binding with name {} does not exist", name))
    }
}


#[cfg(test)]
mod tests {
    use crate::binding_usage::BindingUsage;

    use super::*;
    
    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(110));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string()
            }
            .eval(&env),
            Ok(Val::Number(110))
        )
    }

    #[test]
    fn eval_with_non_existing_binding_usage() {
        let env = Env::default();
        assert_eq!(
            BindingUsage {
                name: "foo".to_string()
            }
            .eval(&env),
            Err("binding with name foo does not exist".to_string())
        )
    }
}