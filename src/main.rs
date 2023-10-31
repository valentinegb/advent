mod years;

use clap::{CommandFactory, Parser};
use years::{yr2015::yr2015, AdventYear};

#[derive(Parser)]
#[command(
    author,
    about,
    override_usage = "advent <YEAR> <DAY> [PART] [INPUT]",
    disable_help_subcommand = true,
    after_help = "To see possible values of a year, run advent <YEAR> --help.\nYou can also run advent <YEAR> <DAY> without [PART] or [INPUT] to see the source code for that day."
)]
struct AdventArgs {
    #[command(subcommand)]
    year: AdventYear,
}

fn main() {
    let advent_args = AdventArgs::parse();
    let mut cmd = AdventArgs::command();

    match advent_args.year {
        AdventYear::Yr2015 { day, part, input } => yr2015(&mut cmd, day, part, input),
    }
}
