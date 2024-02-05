use std::io;
use std::io::Write;

// Takes in numbers separated by space, then outputs the even numbers.
// Inputs: numbers separated by space
// Process: Iterating the numbers, make new list with the even numbers.
// Output: The even numbers.
// Constraints: use vector, dont use builtin filter function

fn filter_even_numbers(numbers: Vec<i64>) -> Vec<i64> {
    // initialize even vector
    let mut even: Vec<i64> = vec![];
    // Iterate numbers
    // if divisible by 2, push to even vector

    for number in numbers {
        if number % 2 == 0 {
            even.push(number);
        }
    }

    // return even vector
    even
}

#[cfg(test)]
mod tests {
    use super::filter_even_numbers;

    #[test]
    fn test_filter_even_numbers() {
        assert_eq!(
            filter_even_numbers(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![2, 4, 6, 8]
        );
        assert_eq!(filter_even_numbers(vec![1, 3, 5, 7]), vec![]);
        assert_eq!(
            filter_even_numbers(vec![-1, -2, -6, -8, -9, 3]),
            vec![-2, -6, -8]
        );

        assert_eq!(filter_even_numbers(vec![0]), vec![0]);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}
fn main() {
    loop {
        // get input, "Enter a list of numbers, separated by spaces: "
        let input: String = get_input("Enter a list of numbers, separated by spaces: ");
        // extract the numbers to a vec of i64
        let split_input: Vec<&str> = input.split_whitespace().collect();
        let result: Result<Vec<i64>, _> = split_input.iter().map(|s| s.parse::<i64>()).collect();
        // if there is not number, show error.
        match result {
            Ok(_) => break,
            Err(_) => println!("There is an invalid number in your input."),
        }
    }

    // get the even numbers
    // print, "The even numbers are: "
}
