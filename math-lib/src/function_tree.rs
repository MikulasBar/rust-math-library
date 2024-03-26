


enum FnParseError {

}

pub struct FnParser {
    custom_fns_def: Option<fn(&str, Vec<ChildFn>) -> Option<ChildFn>>,
    custom_vars_def: Option<fn(&str) -> Option<f64>>
}

impl FnParser {
    pub fn new() -> Self {
        Self {
            custom_fns_def: None,
            custom_vars_def: None,
        }
    }

    pub fn parse_from(&self, input: &str) -> Self {

    }

    pub fn include_functions<T>(&mut self, def: T) -> Self
    where
        T: fn(&str, Vec<ChildFn>) -> Option<ChildFn>
    {
        self.custom_fns_def = def;
    }

    pub fn include_constants<T>(&mut self, def: T) -> Self
    where
        T: fn(&str, Vec<ChildFn>) -> Option<ChildFn>
    {
        self.custom_fns_def = def;
    }

    pub fn build() -> Result<FnTree, FnParseError> {

    }
}



pub struct FnTree {
    definition: ChildFn,
}

impl FnTree {
    pub fn apply(&self, args: &FnArgs) -> FnResult {
        self.definition.apply(args)
    } 
}

impl Default for FnTree {
    fn default() -> Self {
        Self {
            definition: "x".to_child_fn()
        }
    }
}