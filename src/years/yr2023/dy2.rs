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
    todo!();
}
