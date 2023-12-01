pub(super) mod yr2015;
pub(super) mod yr2023;

use clap::{Subcommand, ValueEnum};

use self::{yr2015::Advent2015Day, yr2023::Advent2023Day};

#[derive(Subcommand)]
pub(super) enum AdventYear {
    #[command(name = "2015")]
    Yr2015 {
        /// Day of puzzle
        #[arg(value_enum)]
        day: Advent2015Day,
        /// Puzzle part
        #[arg(value_enum)]
        part: Option<AdventPuzzlePart>,
        /// Puzzle input
        #[arg()]
        input: Option<String>,
    },
    #[command(name = "2023")]
    Yr2023 {
        /// Day of puzzle
        #[arg(value_enum)]
        day: Advent2023Day,
        /// Puzzle part
        #[arg(value_enum)]
        part: Option<AdventPuzzlePart>,
        /// Puzzle input
        #[arg()]
        input: Option<String>,
    },
}

#[derive(ValueEnum, Clone)]
pub(super) enum AdventPuzzlePart {
    #[value(name = "1")]
    Pt1,
    #[value(name = "2")]
    Pt2,
}
