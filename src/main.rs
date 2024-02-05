// Takes in numbers separated by space, then outputs the even numbers.
// Inputs: numbers separated by space
// Process: Iterating the numbers, make new list with the even numbers.
// Output: The even numbers.
// Constraints: use vector, dont use builtin filter function

#[cfg(test)]

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
mod tests {
    use super::*;

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
fn main() {
    println!("Hello, world!");
}
