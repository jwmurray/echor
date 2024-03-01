use anyhow::Result;
use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
struct Args {
    text: Vec<String>,
    omit_newline: bool,
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn get_args() -> Args {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("John Murray <john@gardenway.org>")
        .about("rust version of echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    Args {
        text: matches.get_many("text").unwrap().cloned().collect(),
        omit_newline: matches.get_one("omit_newline").cloned().unwrap(),
    }
}

fn run(args: Args) -> Result<()> {
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
    Ok(())
}
