use anyhow::{Error, Result};
use cli_ck::beep;

fn main() -> Result<(), Error> {
    beep::beep(&4)
}
