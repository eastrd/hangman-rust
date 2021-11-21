use rand::prelude::IteratorRandom;
use std::{
    fs,
    io::{stdin, stdout, Read, Write},
    ops::RangeBounds,
};

fn get_random_word() -> String {
    // open dictionary file and fetch a random word
    let mut content = String::new();
    fs::File::open("dict.txt")
        .expect("error opening dictionary file: dict.txt")
        .read_to_string(&mut content)
        .unwrap();
    loop {
        let w = content
            .split("\n")
            .into_iter()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        if w.len() > 4 {
            return w;
        }
    }
}

fn main() {
    let word = get_random_word();
    let mut good_chars: Vec<char> = Vec::new();
    let mut bad_chars: Vec<char> = Vec::new();
    let mut life: i8 = 10;

    // End loop either ran out of guesses OR guessed everything
    while life > 0
        || word
            .chars()
            .filter(|c| !good_chars.contains(c))
            .collect::<Vec<char>>()
            .len()
            == 0
    {
        // render
        display_gap();
        display_word(&word, &good_chars);
        display_chars(&good_chars, &bad_chars);
        display_life(life);

        // read first character of user input
        let mut buf = String::new();
        print!("Your Guess >> ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut buf)
            .expect("error reading user input");
        let guess = buf.chars().next().unwrap();

        // If guess has already been used, retry input
        if good_chars.contains(&guess) || bad_chars.contains(&guess) {
            println!("[!] You have already guessed '{}' before", guess);
            continue;
        }

        if word.contains(&guess.to_string()) {
            good_chars.push(guess);
        } else {
            bad_chars.push(guess);
            println!("[!] '{}' is NOT in the word\n", guess);
            life -= 1;
        }
    }

    if life == 0 {
        println!("[!!!!!] Game over, the word is >>>'{}'<<<", word);
    } else {
        println!("[!!!!!] WOW, Unbelievable, HOW TF DID YOU GUESS THAT?!");
    }
}

fn display_gap() {
    for _ in 0..30 {
        print!("-");
    }
    println!("");
}

fn display_word(target: &String, goods: &Vec<char>) {
    print!("[WORD]: ");
    target.chars().for_each(|c| {
        if goods.contains(&c) {
            print!("{}", &c)
        } else {
            print!("_");
        }
    });
    println!("");
}

fn display_chars(goods: &Vec<char>, bads: &Vec<char>) {
    let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
    rows.iter().enumerate().for_each(|(idx, row)| {
        for _ in 0..idx {
            print!(" ");
        }
        row.chars().for_each(|c| {
            if goods.contains(&c) || bads.contains(&c) {
                print!(" _");
            } else {
                print!(" {}", c.to_uppercase());
            }
        });
        println!("");
    })
}

fn display_life(life_count: i8) {
    print!("Life: ");
    for _ in 0..life_count {
        print!("♥");
    }
    println!("");
}
