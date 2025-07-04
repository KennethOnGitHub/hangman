use std::{collections::HashSet, fs, io};
const WORD_LIST_PATH: &str = "google-10000-english.txt";
use rand::{self, seq::IndexedRandom};

fn main() {
    let binding = fs::read_to_string(WORD_LIST_PATH)
                             .expect("Word List Not Found!");
    let word_list: Vec<&str> = binding.lines().collect();

    // let word_list: Vec<&str> = fs::read_to_string(WORD_LIST_PATH)
    //                                 .expect("Word List Not Found!")
    //                                 .lines()
    //                                 .collect();

    let target_word = *word_list.choose(&mut rand::rng()).expect("Failed to generate word to guess!");

    let mut lives = 10;

    let mut past_guesses: HashSet<char> = HashSet::new();

    println!("Guess the word! {}", "_".repeat(target_word.chars().count()));

    loop {
        let guessed_char = {
            loop {
                println!("Enter Letter: ");
                let mut user_in = String::new();
                io::stdin()
                .read_line(&mut user_in)
                .expect("Failed to read input!");

                let c = user_in.chars().next();
                match c {
                   Option::None => { //this path is never taken?
                    println!("No character entered!");
                    continue;},

                   Option::Some(c) => {
                    if user_in.trim_end().chars().count() > 1 {
                        println!("More than 1 character entered!");
                        continue;
                    }
                    if !c.is_ascii_alphabetic() { //you MUST check ascii_alphabetic as alphabetic erronously allows for chinese characters
                        println!("Character not alphabetic!");
                        continue;
                    }
                    if past_guesses.contains(&c) {
                        println!("Character already guessed!");
                        continue;
                    }
                    break c;
                   } 
                }
                }
            };

            past_guesses.insert(guessed_char);

            match target_word.contains(guessed_char) {
                true => {
                    let progress: String = target_word
                        .chars()
                        .map(
                            |c| match past_guesses.contains(&c) {
                                true => c,
                                false => '_'})
                        .collect();

                    if progress == target_word {
                        println!("Correct! Word was {progress}");
                        break;
                    }
                    println!("{}", progress)
                    
                },
                false => {
                    lives -= 1;

                    match lives {
                        1.. => println!("Letter not in word! -1 Lives! {lives} lives remaining"),
                        _ => {
                            println!("You lose! Word was {target_word}");
                            break;
                        }
                    }

                    
                }
            }


        }; 
    }
