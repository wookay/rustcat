#[cfg(not(test))]
fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod test {
  #[test]
  fn test_int() {
    assert!(3 == 1+2);
  }
}
