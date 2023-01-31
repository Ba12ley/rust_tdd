use std::{error, result};
type TError = Box<dyn error::Error>; //Box is a smart pointer that points to a heap-allocated value
type TResult<T> = result::Result<T, TError>;

fn main() {
    println!("Hello, world!");
}

fn read_file(p: &str) -> TResult<String> {
    let content = std::fs::read_to_string(p)?;
    Ok(content)
    // unimplemented!() // this is a macro that tells the compiler that this function is not implemented
}

fn split_numbers(s: &str) -> TResult<Vec<i32>> {
    let mut numbers = Vec::new();
    for num in s.split_whitespace() {
        numbers.push(num.parse()?);
    }
    Ok(numbers)
}

fn add_numbers(numbers: Vec<i32>) -> TResult<i32> {
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    Ok(sum)
}

fn write_file(p: &str, content: &str) -> TResult<()> {
    unimplemented!() // this is a macro that tells the compiler that this function is not implemented, like pass in python
}


#[cfg(test)]
mod tests {
    use super::*; // import all the functions from the parent module
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_read_file() {
        let content = read_file("test_data/test_str.txt");
        assert!(content.is_ok());

        if let Ok(content) = content {
            assert_eq!(content, "This will test the reading of a file");
        }
    }
    #[test]
    fn test_split_numbers() {
        let numbers = split_numbers("1 2 3 4 5");
        assert!(numbers.is_ok());
        if let Ok(numbers) = numbers {
            assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
        }
    }

    #[test]
    fn test_add_numbers() {
        let sum = add_numbers(vec![1, 2, 3, 4, 5]);
        assert!(sum.is_ok());
        assert!(sum.unwrap() == 15); //unwrap() returns the value inside the Ok variant
    }
}