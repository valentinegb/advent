use md5::{Digest, Md5};

pub(super) fn pt1(input: String) {
    let mut number: u32 = 0;

    'outer: loop {
        number += 1;

        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", input, number));
        let result = hasher.finalize();

        for i in 0..2 {
            if result[i] != 0 {
                continue 'outer;
            }
        }

        if result[2] >= 16 {
            continue;
        }

        break;
    }

    println!("{number}");
}

pub(super) fn pt2(input: String) {
    let mut number: u32 = 0;

    'outer: loop {
        number += 1;

        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", input, number));
        let result = hasher.finalize();

        for i in 0..3 {
            if result[i] != 0 {
                continue 'outer;
            }
        }

        break;
    }

    println!("{number}");
}
