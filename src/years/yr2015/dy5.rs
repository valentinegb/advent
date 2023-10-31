pub(super) fn pt1(input: String) {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const NAUGHY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let mut nice_strings: u32 = 0;

    for string in input.split('\n') {
        let mut vowels: u32 = 0;
        let mut contains_double_letter = false;
        let mut contains_naughty_string = false;
        let mut previous_letter = None;

        for char in string.chars() {
            if VOWELS.contains(&char) {
                vowels += 1;
            }

            if let Some(previous_letter) = previous_letter {
                if previous_letter == char {
                    contains_double_letter = true;

                    continue;
                }
            }

            previous_letter = Some(char);
        }

        for naughty_string in NAUGHY_STRINGS {
            if string.contains(naughty_string) {
                contains_naughty_string = true;
            }
        }

        if vowels >= 3 && contains_double_letter && !contains_naughty_string {
            nice_strings += 1;
        }
    }

    println!("{nice_strings}");
}

pub(super) fn pt2(input: String) {
    let mut nice_strings: u32 = 0;

    for string in input.split('\n') {
        let mut contains_double_pair = false;
        let mut contains_repeat = false;
        let mut previous_char = None;
        let mut previous_previous_char = None;

        for (i, char) in string.char_indices() {
            if let Some(previous_char) = previous_char {
                if string[(i + 1)..].contains(&format!("{previous_char}{char}")) {
                    contains_double_pair = true;
                }

                if let Some(previous_previous_char) = previous_previous_char {
                    if previous_previous_char == char {
                        contains_repeat = true;
                    }
                }

                previous_previous_char = Some(previous_char);
            }

            previous_char = Some(char);
        }

        if contains_double_pair && contains_repeat {
            nice_strings += 1;
        }
    }

    println!("{nice_strings}");
}
