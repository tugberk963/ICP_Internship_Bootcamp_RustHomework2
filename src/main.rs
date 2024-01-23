use std::io::{self, Write};

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: String) -> WordCounter {
        WordCounter { text }
    }

    fn count_words(&self) -> usize { 
        /*                 
        In task requirements function definition returns usize 
        so 
        I can't return the error message in this function
        if self.text.is_empty() {  
            return "@Error Empty String"
        }
        */ 
        self.text.split_whitespace().count()
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter a text to count the words in it: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("ff");

    if input.trim().is_empty() { // empty string case handled here
        println!("@Error Empty String");
        return;
    }

    let word_counter = WordCounter::new(input);
    println!("Word count in the text you entered: {}", word_counter.count_words());
}
