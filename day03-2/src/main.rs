use std::collections::HashSet;
use std::io;

fn char_to_score(input: char) -> Result<u32, &'static str> {
    let ascii_value = input as u32;
    if ascii_value >= 65 && ascii_value <= 90 {
        return Ok(ascii_value - 38);
    } else if ascii_value >= 97 && ascii_value <= 122 {
        return Ok(ascii_value - 96);
    }

    return Err("Not a valid item");
}

fn main() -> io::Result<()> {
    let mut buffer;
    let mut sum = 0;
    let stdin = io::stdin();
    let mut items: Vec<String> = Vec::with_capacity(3);
    loop {
        buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        let input = buffer.trim();
        if input == "" {
            break;
        }
        items.push(input.to_string());
        if items.len() != 3 {
            continue;
        }
        let first = items.pop().unwrap();
        let mut first_set: HashSet<char> = HashSet::new();
        for c in first.chars() {
            first_set.insert(c);
        }
        let second = items.pop().unwrap();
        let mut second_set: HashSet<char> = HashSet::new();
        for c in second.chars() {
            if first_set.contains(&c) {
                second_set.insert(c);
            }
        }
        let third = items.pop().unwrap();
        for c in third.chars() {
            if second_set.contains(&c) {
                sum += match char_to_score(c) {
                    Ok(i) => i,
                    Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Input: {}", input))),
                };
                break;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
