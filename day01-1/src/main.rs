use std::io;

fn main() -> io::Result<()> {
    let mut buffer;
    let mut largest: u32 = 0;
    let mut current: u32 = 0;
    let stdin = io::stdin();
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer == "\n" {
            if current == 0 {
                break;
            }
            else if current > largest {
                largest = current;
            }
            current = 0;
            continue;
        } else if buffer == "" {
            break;
        }
        let input: u32 = buffer.trim().parse().expect("Wanted a number");
        current += input;
    }
    println!("{}", largest);
    Ok(())
}
