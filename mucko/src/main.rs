mod mucko;

fn main() {
  use mucko::base;
  base::println(1+2);
}


#[cfg(test)]
mod tests {
  use main;

  #[test]
  fn mucko_base() {
    main();
  }
} // mod tests
