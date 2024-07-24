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
        let key_characters = Self::filter_characters(&alphabet_characters, key);
        let shifted_alphabet = Self::shift_alphabet(&alphabet_characters, &key_characters);

        Vigenere {
            alphabet: alphabet_characters,
            key: key_characters,
            shifted_alphabet,
        }
    }

    pub fn encrypt(&self, text: &str) -> String {
        let text_characters = Self::filter_characters(&self.alphabet, text);
        let enlarged_key = self.enlarge_key(text.len());
        // Encrypted vector
        let mut vec: Vec<char> = Vec::new();

        // Iterate over text characters
        for (pos, c) in text_characters.iter().enumerate() {
            // Get for character the index in alphabet
            let index = self.alphabet.iter().position(|&x| x.eq(&c));

            // Get key at character position
            let key = enlarged_key.get(pos);
            // Get shifted alphabet for key
            let found = self.shifted_alphabet.get(&key.unwrap());

            // Check valid alphabet
            if found.is_some() && index.is_some(){
                let alphabet_for_key= found.unwrap();
                let encrypted = alphabet_for_key.get(index.unwrap());
                vec.push(encrypted.unwrap().to_owned());
            }
        }

        let s: String = vec.into_iter().collect();
        s
    }

    pub fn decrypt(&self, text: &str) -> String {
        let text_characters = Self::filter_characters(&self.alphabet, text);
        let enlarged_key = self.enlarge_key(text.len());
        // Decrypted vector
        let mut vec: Vec<char> = Vec::new();
        let s: String = vec.into_iter().collect();

        s
    }

    fn filter_characters(alphabet: &Vec<char>, text: &str) -> Vec<char> {
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
        assert_eq!(encrypted, "jvkjbvrpgirpssisim");
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
