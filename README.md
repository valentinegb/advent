# `advent`

CLI for running Advent of Code puzzles that I've solved

## What?

This is a project I've made to organize all of the Advent of Code puzzles that I've completed. It's a command line interface, and it's usage is like this:

```
advent <YEAR> <DAY> [PART] [INPUT]
```

So running a puzzle in practice looks like this:

```shell
$ advent 2015 3 1 "^>v<"
4
```

You can see the source code for a day without opening up an editor by not specifying `[PART]` or `[INPUT]`, like this:

```shell
$ advent 2015 4
```

```rust
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
```

Run `advent --help` for more information.

## Why?

I wanted to start keeping the puzzles I've been completing so that I can look back at them later and see how I've improved. Also, if anyone else doing Advent of Code puzzles would like to, they can ~~copy and paste~~ take inspiration from my code.

Joking aside, please don't look at my puzzle solutions unless you've already done the puzzle yourself or you don't plan on doing it. I find that to be against the spirit of Advent of Code. Challenge yourself! Copying and pasting isn't very challenging, anyone can do that.
