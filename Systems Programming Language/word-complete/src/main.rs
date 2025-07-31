// Starter file provided to CSC 330, Summer 2025, Assignment 3
// Copyright Mike Zastre, UVic 2025.
//
// This echoes the functionality provided by the starter file in
// Haskell for the similar problem in Assignment 1.
//
// Therefore your task is to complete the functionality needed
// by `min_keystrokes()` -- and which will (perhaps) including writing
// other Rust functions in turn.
//

use std::fs::read_to_string;
use std::env;


// Calculate the cost of transforming one word to another
// by backspacing and typing new characters
fn transform_cost(from: &str, to: &str) -> i32 {
    // Find the longest common prefix
    let common_prefix_len = from.chars()
        .zip(to.chars())
        .take_while(|(a, b)| a == b)
        .count();
    
    // Cost = backspaces to remove non-matching suffix + keypresses to type new suffix
    // Use .chars().count() instead of .len() to handle Unicode characters correctly
    let backspaces = from.chars().count() - common_prefix_len;
    let new_chars = to.chars().count() - common_prefix_len;
    
    (backspaces + new_chars) as i32
}

fn min_keystrokes(from_word: &str, to_word: &str, words: Vec<&str>) -> i32
{
    // Option 1: Transform directly from current word to target word
    let direct_cost = transform_cost(from_word, to_word);
    
    // Option 2: Select one of the suggested words, then transform to target
    let mut min_suggestion_cost = i32::MAX;
    
    for suggestion in &words {
        // 1 keypress to select the suggestion + cost to transform suggestion to target
        let suggestion_cost = 1 + transform_cost(suggestion, to_word);
        min_suggestion_cost = min_suggestion_cost.min(suggestion_cost);
    }
    

    
    // Return the minimum of direct transformation or using suggestions
    direct_cost.min(min_suggestion_cost)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return;
    }

    let contents: String = read_to_string(&args[1])
        .expect("Should have been able to read the file.");

    // What follows makes the assumption that the content
    // of an input file is properly formed. If we wanted to
    // write error code to handle input-file issues, then
    // much more would be needed...
    //

    let mut lines = contents.lines();
    let test_count_line = lines.next().unwrap();
    let test_count: i32 = test_count_line.parse().unwrap();

    for _ in 0..test_count {
        let to_word = lines.next().unwrap();
        let from_word = lines.next().unwrap();

        let mut words = Vec::new();
        for _ in 0..3 {
            let word = lines.next().unwrap();
            words.push(word);
        }

        let min_keystrokes = min_keystrokes(from_word, to_word, words);
        println!("{}", min_keystrokes);
    }

}

