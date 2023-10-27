pub(super) mod yr2015;

use clap::{Subcommand, ValueEnum};

use self::yr2015::Advent2015Day;

#[derive(Subcommand)]
pub(super) enum AdventYear {
    #[command(name = "2015")]
    Yr2015 {
        /// Day of puzzle
        #[arg(value_enum)]
        day: Advent2015Day,
        /// Puzzle part
        #[arg(value_enum)]
        part: AdventPuzzlePart,
        /// Puzzle input
        #[arg()]
        input: String,
    },
}

#[derive(ValueEnum, Clone)]
pub(super) enum AdventPuzzlePart {
    #[value(name = "1")]
    Pt1,
    #[value(name = "2")]
    Pt2,
}
