use stop_words;

fn main() {
    // Get the stop words
    let words = stop_words::get("eng");

    // Print them
    for word in words {
        println!("{}", word)
    }
}
