use std::collections::HashMap;

use builder::Builder;


pub struct Context<'a> {
    vars: HashMap<&'a str, f64>,
}

impl<'a> Context<'_> {
    pub fn builder() -> Builder<'a> {
        Builder {
            vars: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: impl Into<&'a str>) -> Option<f64> {
        self.vars.get(name.into()).copied()
    }
}

impl<'a> From<HashMap<&'a str, f64>> for Context<'a> {
    fn from(vars: HashMap<&'a str, f64>) -> Self {
        Context {
            vars,
        }
    }
}

mod builder {
    use std::collections::HashMap;
    use super::Context;

    pub struct Builder<'a> {
        pub vars: HashMap<&'a str, f64>,
    }

    impl<'a> Builder<'a> {
        pub fn add_const(mut self, name: &'a str, value: f64) -> Self {
            self.vars.insert(name, value);
            self
        }

        pub fn set_consts(mut self, consts: HashMap<&'a str, f64>) -> Self {
            self.vars = consts;
            self
        }

        pub fn build(self) -> Context<'a> {
            Context {
                vars: self.vars,
            }
        }
    }
}