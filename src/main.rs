mod years;

use clap::Parser;
use years::{yr2015::yr2015, AdventYear};

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    override_usage = "advent <YEAR> <DAY> <PART> <INPUT>",
    disable_help_subcommand = true,
    after_help = "To see possible values of a year, run `advent <YEAR> --help`."
)]
struct AdventArgs {
    #[command(subcommand)]
    year: AdventYear,
}

fn main() {
    let advent_args = AdventArgs::parse();

    match advent_args.year {
        AdventYear::Yr2015 { day, part, input } => yr2015(day, part, input),
    }
}
