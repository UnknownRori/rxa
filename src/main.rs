use anyhow::Result;
use clap::Parser;

use rxa::{App, Args};

fn main() -> Result<()> {
    let args = Args::parse();
    let app = App::new(args);
    app.run()?;

    Ok(())
}
