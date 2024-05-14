#![allow(dead_code, unused_variables)]

fn main() {
    let phrase = "This is my clear text before encoding it";
    let letters = phrase.chars().collect::<Vec<char>>();

    let src4 = encode(&phrase);
    let char4: Vec<char> = src4.chars().collect();

    let mut buff = String::new();
    let mut output: Vec<u8> = Vec::new();

    for l in char4 {
        buff.push(l);
        // print!("{} -> ", buff);
        match buff.len() {
            // 1 => (), //println!("need more feed"),
            2 | 3 => {
                let value: u8 = buff.parse().unwrap();
                match value {
                    32 | 65..=90 | 97..=122 => {
                        output.push(value);
                        buff.clear();
                    }
                    _ => (),
                }
            }
            _ => (), //println!("buffer too long!"),
        }

        // print!("{}", l);
    }

    let sentence = String::from_utf8(output).unwrap_or("could not decode input".to_string());

    println!("Decoded input is: {}", sentence);
}

fn encode(input: &str) -> String {
    let letters = input.chars().collect::<Vec<char>>();
    let byte1: Vec<u8> = letters.iter().map(|c| *c as u8).collect::<Vec<_>>();

    let mut buff: String = String::new();
    for i in byte1 {
        let c = format!("{}", i);
        buff.push_str(c.as_str());
    }

    buff
}
