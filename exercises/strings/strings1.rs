// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

use std::hint;

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // String::from("blue")
    let mut fav_col = String::new();
    let data = "blue";
    fav_col = data.to_string();
    fav_col
}
