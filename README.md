# rust-vigenere
Library for encrypting and decrypting a text with the Vigenère chiffre.  
This library is operating on UTF-8 characters and therefore works with greek or japanese letters.

## Usage

```rust
const ALPHABET_GREEK: &'static str = "αβγδεζηθικλμνξοπρστυφχψω";
// Create key
let key = "ολυμπια"; // olympia

// Create vigenere with greek alphabet
let vigenere = Vigenere::new(key, Some(ALPHABET_GREEK));

// Encrypt
let encrypted = vigenere.encrypt("μεσανυχτα");

// Decrypt
let decrypted = vigenere.decrypt("βονμδδχιλ");
```

## References
Vigenere cipher https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
