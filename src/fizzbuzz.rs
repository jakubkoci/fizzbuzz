pub mod fizzbuzz {
  pub fn fizzbuzz(number: u32) -> String {
    let is_divisible_by_3 = is_divisible_by(3, number);
    let is_divisible_by_5 = is_divisible_by(5, number);

    let mut result = String::from(number.to_string());
    if is_divisible_by_3 || is_divisible_by_5 {
      result = String::new();
      if is_divisible_by_3 {
        result.push_str("fizz");
      }

      if is_divisible_by_5 {
        result.push_str("buzz");
      }
    }

    return result;
  }

  fn is_divisible_by(divider: u32, number: u32) -> bool {
    return number % divider == 0;
  }

  #[test]
  fn with_1_returns_number() {
    assert_eq!(fizzbuzz(1), "1");
  }

  #[test]
  fn with_2_returns_number() {
    assert_eq!(fizzbuzz(2), "2");
  }

  #[test]
  fn with_3_returns_fizz() {
    assert_eq!(fizzbuzz(3), "fizz");
  }

  #[test]
  fn with_5_returns_buzz() {
    assert_eq!(fizzbuzz(5), "buzz");
  }

  #[test]
  fn with_6_returns_fizz() {
    assert_eq!(fizzbuzz(6), "fizz");
  }

  #[test]
  fn with_10_returns_buzz() {
    assert_eq!(fizzbuzz(10), "buzz");
  }

  #[test]
  fn with_15_returns_fizzbuzz() {
    assert_eq!(fizzbuzz(15), "fizzbuzz");
  }
}
