mod api;
mod git;
mod util;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    Ok(())
}
