mod dy1;
mod dy2;
mod dy3;

use clap::ValueEnum;

use super::AdventPuzzlePart;

#[derive(ValueEnum, Clone)]
pub(crate) enum Advent2015Day {
    #[value(name = "1")]
    Dy1,
    #[value(name = "2")]
    Dy2,
    #[value(name = "3")]
    Dy3,
}

pub(crate) fn yr2015(day: Advent2015Day, part: AdventPuzzlePart, input: String) {
    match day {
        Advent2015Day::Dy1 => match part {
            AdventPuzzlePart::Pt1 => dy1::pt1(input),
            AdventPuzzlePart::Pt2 => dy1::pt2(input),
        },
        Advent2015Day::Dy2 => match part {
            AdventPuzzlePart::Pt1 => dy2::pt1(input),
            AdventPuzzlePart::Pt2 => dy2::pt2(input),
        },
        Advent2015Day::Dy3 => match part {
            AdventPuzzlePart::Pt1 => dy3::pt1(input),
            AdventPuzzlePart::Pt2 => dy3::pt2(input),
        },
    }
}
