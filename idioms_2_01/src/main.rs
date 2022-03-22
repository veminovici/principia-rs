// Use borrowed types for arguments
// Prefer using the borrowed type over borrowing the owned type.
// &str over &String
// &[T] over &Vec<T>
// &T   over &Box<T>

// Determine if a word contains three consecutive vowels.
// We don't need to own the string to determine this, so we will take a reference.

/// Use &str over &String
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }

    false
}

fn main() {
    let sentence_string = "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels", word);
        }
    }
}
