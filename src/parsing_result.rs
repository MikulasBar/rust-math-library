use derive_more::Display;
use crate::{child::Child, function::Function};




/// this exists only because the antlr visitor pattern trait only accepts
/// Return types only that have `Default` implementation <br>
/// it is converted to normal `Result<Child, ParsingError>` in `ParserFn` struct
// #[derive(Debug)]
pub enum ParsingResult {
    Ok(Child),
    Err(ParsingError),
}


impl ParsingResult {

    // pub fn ok(self) -> Option<Child> {
    //     match self {
    //         Self::Ok(v) => Some(v),
    //         _ => None
    //     }
    // }

    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub fn unwrap(self) -> Child {
        match self {
            Self::Ok(v) => v,
            _ => panic!("Err cannot be unwraped")
        }
    }
}

impl Default for ParsingResult {
    fn default() -> Self {
        Self::Err(ParsingError::Placeholder)
    }
}

impl Into<Result<Child, ParsingError>> for ParsingResult {
    fn into(self) -> Result<Child, ParsingError> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::Err(e) => Err(e)
        }
    }
}

impl From<Child> for ParsingResult {
    fn from(c: Child) -> Self {
        Self::Ok(c)
    }
}

impl From<Function> for ParsingResult {
    fn from(f: Function) -> Self {
        Child::from(f).into()
    }
}

// impl From<ParsingError> for ParsingResult {
//     fn from(e: ParsingError) -> Self {
//         Self::Err(e)
//     }
// }

impl From<f64> for ParsingResult {
    fn from(f: f64) -> Self {
        Child::from(f).into()
    }
}

impl From<String> for ParsingResult {
    fn from(s: String) -> Self {
        Child::from(s).into()
    }
}




#[derive(Debug, Display)]
pub enum ParsingError {
    Placeholder,
    Antlr
}