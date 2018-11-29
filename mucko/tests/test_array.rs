extern crate mucko;

#[cfg(test)]
mod test_array {
  use mucko::base::{pushi};

  #[test]
  fn test_pushi() {
    let mut a = vec![1,2];
    pushi(&mut a, 3);
    assert_eq!(a, [1,2,3]);
  }
} // mod test_array
