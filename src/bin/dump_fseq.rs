use anyhow::{anyhow, Result};
use clap::{Arg, Command};

fn main() -> Result<()> {
    let args = Command::new("dump_fseq")
        .arg(Arg::new("filename"))
        .get_matches();

    match args.get_one::<String>("filename") {
        Some(filename) => match ledplayr::fseq::parser::parse(filename) {
            Ok(fseq) => {
                println!("{}", fseq);
                Ok(())
            }
            Err(e) => Err(e),
        },
        None => Err(anyhow!("Missing argument")),
    }
}
