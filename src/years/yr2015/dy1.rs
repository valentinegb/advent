pub(super) fn pt1(input: String) {
    let mut floor = 0;

    for char in input.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!("puzzle input should not contain characters other than '(' and ')'"),
        }
    }

    println!("{floor}");
}

pub(super) fn pt2(input: String) {
    let mut floor = 0;

    for (index, char) in input.char_indices() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!("puzzle input should not contain characters other than '(' and ')'"),
        }

        if floor < 0 {
            return println!("{}", index + 1);
        }
    }
}
