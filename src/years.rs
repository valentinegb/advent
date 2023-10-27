pub(super) mod yr2015;

use clap::Subcommand;

use self::yr2015::Advent2015Day;

#[derive(Subcommand)]
pub(super) enum AdventYear {
    #[command(name = "2015")]
    Yr2015 {
        /// Day of puzzle
        #[arg(value_enum)]
        day: Advent2015Day,
        /// Puzzle input
        #[arg()]
        input: String,
    },
}
