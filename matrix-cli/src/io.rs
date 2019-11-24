use std::io;
use std::io::Write;

pub fn request_input(label: &str, buffer: &mut String) {
  print!("{}: ", label);
  io::stdout().flush().unwrap();
  match io::stdin().read_line(buffer) {
    Ok(_) => *buffer = buffer.trim().to_string(),
    Err(_) => println!("failed to read {}", label),
  }
}
