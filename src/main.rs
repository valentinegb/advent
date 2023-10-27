mod years;

use clap::Parser;
use years::{yr2015::yr2015, AdventYear};

#[derive(Parser)]
#[command(version, author, about)]
struct AdventArgs {
    #[command(subcommand)]
    year: AdventYear,
}

fn main() {
    let advent_args = AdventArgs::parse();

    match advent_args.year {
        AdventYear::Yr2015 { day, input } => yr2015(day, input),
    }
}
