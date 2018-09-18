use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  let loud = input.to_uppercase();
  io::stdout().write(&loud.as_bytes())?;
  Ok(())
}

