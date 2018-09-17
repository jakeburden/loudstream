use std::io;

fn main() {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      print!("{}", input.to_uppercase());
    },
    Err(error) => println!("error: {}", error),
  }
}

