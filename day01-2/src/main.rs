use reduce::Reduce;
use std::collections::LinkedList;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer;
    let mut current: u32 = 0;
    let mut items: LinkedList<u32> = LinkedList::new();
    let stdin = io::stdin();
    loop {
        buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if buffer == "\n" {
            if current == 0 {
                break;
            }
            let largest: u32 = match items.front() {
                Some(i) => *i,
                None => 0,
            };
            if current >= largest {
                items.push_front(current);
            } else {
                items.push_back(current);
            }
            current = 0;
            continue;
        } else if buffer == "" {
            break;
        }
        let input: u32 = match buffer.trim().parse() {
            Ok(t) => t,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{}", e))),
        };
        current += input;
    }
    let mut sorted_items: Vec<u32> = Vec::with_capacity(items.len());
    while items.front() != None {
        let item: u32 = match items.pop_front() {
            Some(i) => i,
            None => break,
        };
        sorted_items.push(item);
    }
    sorted_items.sort();
    let split_index = match sorted_items.len() >= 3 {
        true => sorted_items.len() - 3,
        false => 0,
    };
    let max_items = sorted_items.split_off(split_index);
    if max_items.len() < 1 {
        println!("0");
        return Ok(());
    }
    let sum = match Reduce::reduce(max_items.into_iter(), |a, b| a + b) {
        Some(i) => i,
        None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown Input")),
    };
    println!("{}", sum);
    Ok(())
}
