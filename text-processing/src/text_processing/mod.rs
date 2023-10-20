mod letters;
mod numbers;

pub fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = letters::count_letters(text);
    let number_of_numbers = numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}