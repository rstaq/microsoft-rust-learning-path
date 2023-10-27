use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    println!("Hungry: {}", get_me_some_food(2));
    println!("Longest word: {}", longest_word(&String::from("greater"), &String::from("short")))
}


fn get_me_some_food(index: usize) -> String {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    let mut coco = false;
    if let Some(&"coconut") = fruits.get(index) {
        coco = true;
    };

    match fruits.get(index) {
        Some(fruit_name) => fruit_name.to_string(),
        None => "Today no fruits available".to_string(),
    }
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
