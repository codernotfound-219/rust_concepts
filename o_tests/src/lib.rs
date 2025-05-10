pub fn add(left: u8, right: u8) -> u8 {
    left + right
}

pub struct Guess { value: i32 }

impl Guess {
    pub fn new_guess(val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!(
                "Guess value must be between 100, got {}",
                val
            );
        }

        Guess { value: val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    // #[should_panic(expected = "Expected a number from 1 to 100")]
    #[should_panic]
    fn icikit() {
        Guess::new_guess(200);
    }

    #[test]
    fn working_result() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("WHAT?"))
        }
    }

    #[test]
    #[ignore]
    fn expensive() {
        let mut count = 1;
        loop {
            count += 1;
            if count == 100000 {
                break;
            }
        }
    }
}
