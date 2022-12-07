use std::convert::TryInto;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer;
    let mut score = 0;
    let stdin = io::stdin();
    loop {
        buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if buffer == "\n" || buffer == "" {
            break;
        }
        let round: [&str; 2] = match buffer.split_whitespace().collect::<Vec<&str>>().try_into() {
            Ok(i) => i,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Input: {}", buffer.trim()))),
        };
        let [opponent, player] = round;
        score += match opponent {
            "A" => match player {
                "X" => 4,
                "Y" => 8,
                "Z" => 3,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Input: {}", buffer.trim()))),
            },
            "B" => match player {
                "X" => 1,
                "Y" => 5,
                "Z" => 9,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Input: {}", buffer.trim()))),
            },
            "C" => match player {
                "X" => 7,
                "Y" => 2,
                "Z" => 6,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Input: {}", buffer.trim()))),
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Input: {}", buffer.trim()))),
        };
    }
    println!("{}", score);
    Ok(())
}
