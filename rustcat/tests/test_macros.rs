macro_rules! hi {
  ( $x:expr ) => { (1,2,$x) }
}

#[cfg(test)]
mod test_macros {
  #[test]
  fn test_macro() {
    assert!( hi!(3) == (1,2,3) );
    assert!( hi!("a") == (1,2,"a") );
  }
}
