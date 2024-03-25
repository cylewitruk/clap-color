use clap::{builder::{styling::{AnsiColor, Effects}, Styles}, Args, ColorChoice, Parser, Subcommand};
use color_eyre::Result;

fn main() -> Result<()> {
    match Cli::try_parse() {
        Ok(cli) => match cli.command {
            MySubCommands::Foo(_) => {
                println!("Running foo");
            }
            MySubCommands::Bar => {
                println!("Running bar");
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(styles = styles(), color = ColorChoice::Always)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: MySubCommands
}

#[derive(Debug, Subcommand)]
pub enum MySubCommands {
    /// This is a foo
    Foo(FooArgs),
    /// This is a bar
    Bar
}

#[derive(Debug, Args)]
pub struct FooArgs;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}
