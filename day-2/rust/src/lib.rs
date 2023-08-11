fn core(input: &str) -> Option<i32> {
    input
        .split('x')
        .into_iter()
        // NOTE: dont care about errors, just bubble up a none for now.
        .map(|x: &str| match i32::from_str_radix(x, 10).ok() {
            Some(val) => val,
            None => return None,
        })
        .collect();
    Some(0)
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{Itertools, Position};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::time::{Duration, Instant};
    use std::{env, fs};

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
