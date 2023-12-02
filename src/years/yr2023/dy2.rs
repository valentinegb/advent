pub(super) fn pt1(input: String) {
    let mut possible_games = vec![];

    'game: for (id, game) in input.split('\n').enumerate() {
        for set in game.split_once(": ").unwrap().1.split("; ") {
            let mut reds = 0;
            let mut greens = 0;
            let mut blues = 0;

            for cubes in set.split(", ") {
                let (quantity, color) = cubes.split_once(' ').unwrap();
                let quantity: u16 = quantity.parse().unwrap();

                match color {
                    "red" => reds += quantity,
                    "green" => greens += quantity,
                    "blue" => blues += quantity,
                    _ => unreachable!(),
                }
            }

            if reds > 12 || greens > 13 || blues > 14 {
                continue 'game;
            }
        }

        possible_games.push(id + 1);
    }

    println!("{}", possible_games.into_iter().sum::<usize>());
}

pub(super) fn pt2(input: String) {
    let mut sum = 0;

    for game in input.split('\n') {
        let (_id, game) = game.split_once(": ").unwrap();

        let mut largest_red = 0;
        let mut largest_green = 0;
        let mut largest_blue = 0;

        for set in game.split("; ") {
            for blocks in set.split(", ") {
                let (quantity, color) = blocks.split_once(' ').unwrap();
                let quantity: u32 = quantity.parse().unwrap();

                match color {
                    "red" => {
                        if quantity > largest_red {
                            largest_red = quantity
                        }
                    }
                    "green" => {
                        if quantity > largest_green {
                            largest_green = quantity
                        }
                    }
                    "blue" => {
                        if quantity > largest_blue {
                            largest_blue = quantity
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        sum += largest_red * largest_green * largest_blue;
    }

    println!("{sum}");
}
