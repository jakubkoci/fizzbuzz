pub mod fizzbuzz {
  pub fn fizzbuzz(number: u32) -> String {
    if number == 3 {
      return "fizz".to_string();
    } else if number == 5 {
      return "buzz".to_string();
    }
    return number.to_string();
  }

  #[test]
  fn returns_number() {
    assert_eq!(fizzbuzz(1), "1");
  }

  #[test]
  fn returns_fizz() {
    assert_eq!(fizzbuzz(3), "fizz");
  }

  #[test]
  fn returns_buzz() {
    assert_eq!(fizzbuzz(5), "buzz");
  }
}
