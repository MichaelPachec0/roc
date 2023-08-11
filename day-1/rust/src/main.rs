//! --- Day 1: Not Quite Lisp ---
//! https://adventofcode.com/2015/day/1
//! Revival of Code: 2023 Day-1
//!

//Santa was hoping for a white Christmas, but his weather machine's "snow"
//function is powered by stars, and he's fresh out! To save Christmas, he
//needs you to collect fifty stars by December 25th.
//
// Collect stars by helping Santa solve puzzles. Two puzzles will be made
// available on each day in the Advent calendar; the second puzzle is unlocked
// when you complete the first. Each puzzle grants one star. Good luck!
//
// Here's an easy puzzle to warm you up.
//
// Santa is trying to deliver presents in a large apartment building, but he
// can't find the right floor - the directions he got are a little confusing.
// He starts on the ground floor (floor 0) and then follows the instructions
// one character at a time.
//
// An opening parenthesis, (, means he should go up one floor, and a closing
// parenthesis, ), means he should go down one floor.
//
// The apartment building is very tall, and the basement is very deep; he
// will never find the top or bottom floors.

fn main() {
    println!("Hello, world!");
}
fn example(input: &str) -> i32 {
    input
        .chars()
        .fold(0, |acc, x| if x == '(' { acc + 1 } else { acc - 1 })
}
fn part_two(input: &str) -> Option<u32> {
    let mut acc = 0;
    for (level, ch) in input.chars().enumerate() {
        if ch == '(' {
            acc += 1;
        } else {
            acc -= 1;
        }
        if acc == -1 {
            // NOTE: remember that enumerate starts at 0, we want the Position
            //     (which starts at 1) so add that here
            return Some((level as u32) + 1);
        }
    }
    return None;
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
    #[test]
    fn example_test() {
        // For example:
        // (()) and ()() both result in floor 0.
        // ((( and (()(()( both result in floor 3.
        // ))((((( also results in floor 3.
        // ()) and ))( both result in floor -1 (the first basement level).
        // ))) and )())()) both result in floor -3.
        let strings = vec![
            "(())", "()()", "(((", "(()(()(", "))(((((", "())", "))(", ")))", ")())())",
        ];
        let solutions = vec![0, 0, 3, 3, 3, -1, -1, -3, -3];
        strings
            .iter()
            .enumerate()
            .zip(solutions)
            .for_each(|((iter, str), sol)| {
                println!("ITERATION {iter}");
                let actual = example(str);
                assert_eq!(actual, sol, "EXPECTED: {sol} ACTUAL: {actual}");
            })
    }
    #[test]
    fn part_1() {
        let strings = reader("../input.txt", None).collect::<Vec<String>>();
        strings.iter().enumerate().for_each(|(iter, str)| {
            let actual = example(str);
            println!("ITER: {iter} awnser: {actual}");
        });
    }
    #[test]
    fn part_two_test() {
        let strings = reader("../input.txt", None).collect::<Vec<String>>();
        strings.iter().enumerate().for_each(|(iter, str)| {
            match part_two(str) {
                Some(val) => println!("ITER: {iter} awnser {val}"),
                // NOTE: There is supposed to be a solution, if not then panic.
                None => panic!("NO SOLUTION!"),
            };
        })
    }
    #[test]
    fn test_cwd() {
        let directory = env::current_dir().unwrap();
        let directory = directory.to_str().unwrap();
        println!("THIS IS THE WAY: {directory}");
    }
}
