use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.get_binding_value_without_err_msg(name)
            .ok_or_else(|| format!("binding with name '{}' does not exist", name))
    }

    pub fn get_binding_value_without_err_msg(&self, name: &str) -> Option<Val> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_value_without_err_msg(name))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::binding_usage::BindingUsage;

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
