use std::{arch::x86_64::_CMP_FALSE_OQ, collections::HashSet, io};

fn main() {
    let target_word = "test";
    let mut lives = 10;

    // let letters = HashSet::from(['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']);

    let mut past_guesses: HashSet<char> = HashSet::new();

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

                    println!("{}", progress)
                    
                },
                false => {
                    println!("Letter not in word! -1 Lives!");
                    lives -= 1;
                }
            }


        }; 
    }
