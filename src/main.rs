use std::{collections::HashMap, fs, io, time::SystemTime};

fn main() {
    println!("Reading file..");
    const PATH: &str = "text.txt";
    let content = fs::read_to_string(PATH).expect("Failed to read content of file, wrong path?");

    let players = player_count(&content);
    let start_time = SystemTime::now();
    println!("Starting hop algorythm");
    let mut results = vec![0; players as usize];
    let chars: Vec<char> = content.chars().collect();
    let letters = letters();
    for i in 0..players {
        results[i as usize] = hop(i, &chars, &letters);
    }

    let time_needed = SystemTime::now()
        .duration_since(start_time)
        .expect("Error measuring needed time for algorythms");
    println!(
        "Done, took: {} ms",
        time_needed.as_millis()
    );
    println!("");

    let min_element = results
        .iter()
        .enumerate()
        .min_by_key(|pair| pair.1)
        .unwrap();

    println!("Player {} won, needed: '{}' hops!", min_element.0 + 1, min_element.1);
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

fn hop(start_index: u32, chars: &[char], letters: &HashMap<char, u32>) -> u32 {
    let mut moves: u32 = 0;

    let ignore_chars = vec![
        ' ', '/', '(', ')', '.', '&', '!', '$', ',', '\n', ':', '%', ';', '-', '_', '=', '{', '}',
        '§', '"', '+', '[', ']', '|', '’', '\'', 
    ];

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

                let next_index = letters.get(letter_lowercase).expect(&format!(
                    "Tried to get char that cannot be found in registered letters: {:?}",
                    letter_lowercase
                ));

                index += next_index;
                moves += 1;
            }
            None => {
                return moves;
            }
        }
    }
}

fn letters() -> HashMap<char, u32> {
    let mut map = HashMap::new();

    let alphabet: Vec<char> = ('a'..='z').collect();

    let mut index = 1;
    for ele in alphabet {
        map.insert(ele, index);
        index += 1;
    }

    map.insert('ä', 27);
    map.insert('ö', 28);
    map.insert('ü', 29);
    map.insert('ß', 30);
    map
}
