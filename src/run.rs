use crate::{note, InstallError};
use duct::Expression;
use std::io::{BufRead, BufReader};

#[macro_export]
macro_rules! run {
    ( $($t:tt)* ) => {
        {
            crate::run::Wrap(duct::cmd!($($t)*))
        }

    };
}

pub struct Wrap(pub Expression);

pub struct WrapWithDescription {
    exp: Expression,
    description: &'static str,
}

impl Wrap {
    pub fn desc(self, description: &'static str) -> WrapWithDescription {
        WrapWithDescription {
            exp: self.0,
            description,
        }
    }
}

impl WrapWithDescription {
    pub fn run(self) -> Result<(), InstallError> {
        note(&format!("--- starting  --- {}", self.description));
        println!("{:?}", self.exp);
        exec(self.exp)?;
        note(&format!("--- done --- {}", self.description));
        Ok(())
    }
}

fn exec(mut exp: Expression) -> Result<(), InstallError> {
    for (key, value) in std::env::vars() {
        exp = exp.env(key, value);
    }

    let reader = exp.stderr_to_stdout().reader()?;
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next() {
        println!("{}", line?);
    }

    Ok(())
}
