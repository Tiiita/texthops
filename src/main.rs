use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::{fs, io, time::SystemTime};

fn main() {
    println!("Reading file..");
    const PATH: &str = "text.txt";
    let content = fs::read_to_string(PATH).expect("Failed to read content of file, wrong path?");

    let players = player_count(&content);
    let start_time = SystemTime::now();
    println!("Starting hop algorythm");

    let results = Arc::new(Mutex::new(vec![0; players as usize]));

    let chars: Vec<char> = content.chars().collect();
    let config = GameConfig::new(&chars);

    (0..players)
        .into_par_iter()
        .for_each(|i| results.clone().borrow_mut().lock().unwrap()[i as usize] = hop(i, &config));

    let time_needed = SystemTime::now()
        .duration_since(start_time)
        .expect("Error measuring needed time for algorythms");
    println!("Done, took: {} ms", time_needed.as_millis());
    println!("");

    let lock = results.lock().unwrap();

    let min_element = lock.iter().enumerate().min_by_key(|pair| pair.1).unwrap();

    println!(
        "Player {} won, needed: '{}' hops!",
        min_element.0 + 1,
        min_element.1
    );
}

fn player_count(content: &String) -> u32 {
    println!("How many players do you want?: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input line..");

    let input: u32 = input.trim().parse().expect("Please type a valid number!");
    if content.len() <= input as usize {
        println!("Too many (text length too short).. Max: {}", content.len());
        return player_count(content);
    }

    println!("Calculating with {} players", input);
    input
}

fn hop(start_index: u32, config: &GameConfig) -> u32 {
    let mut moves: u32 = 0;

    let ignore_chars = &config.ignore_chars;
    let letters = config.letters;
    let chars = config.chars;

    let mut index = start_index;
    loop {
        let char_at_index = &chars.get(index as usize);

        match char_at_index {
            Some(ele) => {
                if ignore_chars.contains(&ele) || ele.is_numeric() {
                    index += 1;
                    continue;
                }

                let letter_lowercase = &ele.to_lowercase().next().unwrap();

                let mut buf = [0_u16; 1];
                letter_lowercase.encode_utf16(&mut buf);
                let next_index = letters[buf[0] as usize];

                index += next_index;
                moves += 1;
            }
            None => {
                return moves;
            }
        }
    }
}

fn letters() -> [u32; 256] {
    let mut buffer = [0_u16; 1];
    let mut a = [0; 256];

    let alphabet: Vec<char> = ('a'..='z').collect();

    let mut index = 1;
    for ele in alphabet {
        ele.encode_utf16(&mut buffer);
        a[buffer[0] as usize] = index;
        index += 1;
    }

    'ä'.encode_utf16(&mut buffer);
    a[buffer[0] as usize] = 27;

    'ö'.encode_utf16(&mut buffer);
    a[buffer[0] as usize] = 28;
    'ü'.encode_utf16(&mut buffer);
    a[buffer[0] as usize] = 29;
    'ß'.encode_utf16(&mut buffer);
    a[buffer[0] as usize] = 30;
    a
}

struct GameConfig<'a> {
    chars: &'a [char],
    letters: [u32; 256],
    ignore_chars: Vec<char>,
}

impl<'a> GameConfig<'a> {
    fn new(chars: &'a [char]) -> Self {
        let letters = letters();
        let ignore_chars = vec![
            ' ', '/', '(', ')', '.', '&', '!', '$', ',', '\n', ':', '%', ';', '-', '_', '=', '{',
            '}', '§', '"', '+', '[', ']', '|', '’', '\'',
        ];
        Self {
            chars,
            letters,
            ignore_chars,
        }
    }
}
