use crate::utils::*;
use crate::InstallError;
use duct::Expression;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

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
    description: String,
}

impl Wrap {
    pub fn desc<'a>(self, description: impl Into<Cow<'a, str>>) -> WrapWithDescription {
        WrapWithDescription {
            exp: self.0,
            description: description.into().into_owned(),
        }
    }

    pub fn run(self) -> Result<(), InstallError> {
        exec(self.0)
    }
}

impl WrapWithDescription {
    pub fn run(self) -> Result<(), InstallError> {
        note(&format!("--- starting  --- {}", self.description));
        exec(self.exp)?;
        note(&format!("--- done --- {}", self.description));
        Ok(())
    }

    pub fn to_file(self, path: impl AsRef<Path>) -> Result<(), InstallError> {
        note(&format!("--- starting  --- {}", self.description));
        note(&format!(
            "will save the result to file: {:?}",
            path.as_ref()
        ));
        exec_to_file(self.exp, path)?;
        note(&format!("--- done --- {}", self.description));
        Ok(())
    }
}

#[must_use]
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

#[must_use]
fn exec_to_file(mut exp: Expression, path: impl AsRef<Path>) -> Result<(), InstallError> {
    for (key, value) in std::env::vars() {
        exp = exp.env(key, value);
    }

    let output = exp.stdout_capture().run()?;

    if output.stdout.len() == 0 {
        return Err(InstallError::EmptyResponse);
    }

    let mut file = File::create(path)?;
    file.write_all(&output.stdout)?;

    Ok(())
}
