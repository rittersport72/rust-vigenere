use std::collections::HashMap;

// Greek alphabet
pub const ALPHABET_GREEK: &'static str = "αβγδεζηθικλμνξοπρστυφχψω";
// ASCII alphabet
pub const ALPHABET_ASCII: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub struct Vigenere {
    alphabet: Vec<char>,
    key: Vec<char>,
    shifted_alphabet: HashMap<char, Vec<char>>,
}

impl Vigenere {
    pub fn new(key: &str) -> Self {
        // Create vectors for alphabet and for key
        let alphabet_characters = ALPHABET_ASCII.chars().collect();
        let key_characters = Self::character_filter(&alphabet_characters, key);
        let shifted_alphabet = Self::shift_alphabet(&alphabet_characters, &key_characters);

        Vigenere {
            alphabet: alphabet_characters,
            key: key_characters,
            shifted_alphabet,
        }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let text_characters = Self::character_filter(&self.alphabet, text);
        let text_length = text_characters.len();
        let enlarged_key = self.enlarge_key(text_length);

        println!("key {:?}", enlarged_key);
        println!("shifted_alphabet {:?}", self.shifted_alphabet.len());
        // TODO encrypt

        "".to_owned()
    }

    pub fn decrypt(&self, text: &str) -> String {
        let text_characters = Self::character_filter(&self.alphabet, text);
        let text_length = text_characters.len();
        let enlarged_key = self.enlarge_key(text_length);

        println!("key {:?}", enlarged_key);
        println!("shifted_alphabet {:?}", self.shifted_alphabet.len());
        // TODO decrypt

        "".to_owned()
    }

    fn character_filter(alphabet: &Vec<char>, text: &str) -> Vec<char> {
        let mut vec = Vec::new();

        for c in text.chars() {
            if alphabet.contains(&c) {
                vec.push(c);
            }
        }
        vec
    }

    fn enlarge_key(&self, text_length: usize) -> Vec<char> {
        let mut enlarged_key = self.key.clone();
        let key_length = enlarged_key.len();

        // Enlarge key vector to fit text
        while enlarged_key.len() <= text_length {
            enlarged_key.extend_from_within(0..key_length);
        }
        enlarged_key
    }

    fn shift_alphabet(alphabet: &Vec<char>, key: &Vec<char>) -> HashMap<char, Vec<char>> {
        let mut key_clone = key.clone();
        // Remove duplicates in key vector
        key_clone.sort();
        key_clone.dedup();

        let mut shifted_alphabet = HashMap::new();

        // Iterate over key characters
        for c in key_clone {
            let result = alphabet.iter().position(|&x| x.eq(&c));
            let split = alphabet.split_at(result.unwrap());

            //println!("{:?} ZZZZZZZZZZZZZZZZ {:?}", split.0, split.1);
            // Shift alphabet beginning with c
            let mut shift_vec = Vec::new();
            shift_vec.append(&mut split.1.to_vec());
            shift_vec.append(&mut split.0.to_vec());

            println!("shifted {:?}", shift_vec);

            shifted_alphabet.insert(c, shift_vec);
        }
        shifted_alphabet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_encrypt() {
        let key = "vigenere";
        let vigenere = Vigenere::new(key);

        let text = "one for all, all for one.";
        let encrypted = vigenere.encrypt(text);
        assert_eq!(encrypted, "");
    }

    #[test]
    fn check_decrypt() {
        let key = "vigenere";
        let vigenere = Vigenere::new(key);

        let text = "one for all, all for one.";
        let decrypted = vigenere.decrypt("blablabla");
        assert_eq!(decrypted, "");
    }
}
