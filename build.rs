use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  shadow_rs::new()?;

  Ok(())
}
