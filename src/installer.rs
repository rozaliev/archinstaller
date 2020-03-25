use crate::run;
use crate::{confirm, note, InstallError};

pub(crate) fn install() -> Result<(), InstallError> {
    confirm("Are you connected to Internet")?;

    run!("ip", "link").desc("Current network settings").run()?;

    Ok(())
}
