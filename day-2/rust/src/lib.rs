fn core(input: &str) -> Option<i32> {
    let mut present_box: [i32; 3] = [0; 3];
    let mut min = i32::MAX;
    for (i, str) in input.split('x').into_iter().enumerate() {
        let side = i32::from_str_radix(str, 10).ok()?;
        // WARN: lets be "safe", since there are only 3 possible entries
        if i < 3 {
            present_box[i] = side;
        } else {
            return None;
        }
    }
    // NOTE: do half of the calcuation for finding the surface area of the box.
    present_box = [
        present_box[0] * present_box[1],
        present_box[1] * present_box[2],
        present_box[0] * present_box[2],
    ];
    let ret = present_box.into_iter().fold(0, |acc, side| {
        if side < min {
            min = side
        };
        // NOTE: here we do the last half of the surface area calculation.
        acc + (side * 2)
    });
    Some(ret + min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{Itertools, Position};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::time::{Duration, Instant};
    use std::{env, fs};

    #[test]
    fn example_test() {
        let tests: [&str; 2] = ["2x3x4", "1x1x10"];
        let solutions: [i32; 2] = [58, 43];
        tests
            .into_iter()
            .enumerate()
            .zip(solutions)
            .for_each(|((i, str), sol)| {
                let check = core(str);
                println!("ITER {i}");
                match check {
                    Some(actual) => {
                        assert_eq!(actual, sol, "ERROR: EXPECTED {sol} NEQL ACTUAL: {actual}");
                    }
                    None => panic!("ERROR in processing {str}"),
                }
            })
    }

    /// Helper function to return a File Buffer. Used to isolate imperative code from the
    /// codebase
    fn reader_helper(path: &str) -> BufReader<File> {
        // I am fine with it panicking here, all of the code depends on these lines
        // TODO: might figure out later a better way to refactor this code.
        let input: File = File::open(path).map_or_else(
            // NOTE: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or_else
            // makes it easy to handle errors since the default condition will
            // be the Err varriant
            |_| {
                panic!("File {path} cannot be read");
            },
            |x| x,
        );
        BufReader::new(input)
    }

    /// Main file opening code path. Written as an generic Iterator (that returns a String for every line)
    /// so that it can be chained with other methods.
    fn reader<'args_life>(
        path: &str,
        pattern: Option<&'args_life str>,
    ) -> impl Iterator<Item = String> + 'args_life {
        reader_helper(path)
            .lines()
            .into_iter()
            .map(Result::unwrap_or_default)
            // the filter below should not consume the String being passed down, instead use a reference,
            // what does need to be consumed is the pattern variable.
            .filter(move |line: &String| {
                // TODO: dont consume by default. Only do it when pattern is set.
                // TODO: decide if implemtenting a multi-pattern match make sense.
                !(line.is_empty() || line.contains(pattern.unwrap_or("//")))
            })
    }
}
