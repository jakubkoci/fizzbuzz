pub mod fizzbuzz {
  pub fn fizzbuzz(number: u32) -> String {
    if number % 3 == 0 {
      return "fizz".to_string();
    } else if number % 5 == 0 {
      return "buzz".to_string();
    }
    return number.to_string();
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
}
