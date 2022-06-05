use crate::val::Val;
use std::collections::HashMap;
use crate::stmt::Stmt;

#[derive(Debug, Clone, PartialEq)]
pub enum NamedInfo {
    Binding(Val),
    Func { params: Vec<String>, body: Stmt }
}

impl NamedInfo {
    fn into_bindings(self) -> Option<Val> {
        if let Self::Binding(val) = self {
            Some(val)
        } else {
            None
        }
    }

    fn into_func(self) -> Option<(Vec<String>, Stmt)> {
        if let Self::Func {params, body} = self {
            Some((params, body))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    named: HashMap<String, NamedInfo>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub fn create_child(&'parent self) -> Self {
        Self {
            named: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn store_binding(&mut self, name: String, val: Val) {
        self.named.insert(name, NamedInfo::Binding(val));
    }

    pub fn store_func(&mut self, name: String, params: Vec<String>, body: Stmt) {
        self.named.insert(name, NamedInfo::Func {params, body});
    }

    pub fn get_binding(&self, name: &str) -> Result<Val, String> {
        self.get_named_info(name)
            .and_then(NamedInfo::into_bindings)
            .ok_or_else(|| format!("binding with name '{}' does not exist", name))
    }

    pub fn get_func(&self, name: &str) -> Result<(Vec<String>, Stmt), String> {
        self.get_named_info(name)
            .and_then(NamedInfo::into_func)
            .ok_or_else(|| format!("function with name '{}' does not exist", name))
    }

    pub fn get_named_info(&self, name: &str) -> Option<NamedInfo> {
        self.named.get(name).cloned().or_else(|| {
            self.parent.and_then(|parent| parent.get_named_info(name))
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
            Err("binding with name 'foo' does not exist".to_string())
        )
    }
}
