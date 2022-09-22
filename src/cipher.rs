/*
Copyright 2022 Rodrigo Santiago.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
*/

type VigMatrix = [[char; 26]; 26];

// Returns the index value of a char in the Vigenere matrix
fn idx_finder(ch: char) -> Option<usize> {
    for (idx, chi) in ('A'..='Z').enumerate() {
        if ch == chi {
            return Some(idx);
        }
    }
    None
}

// Returns the char value of an index in the Vigenere matrix
fn char_finder(idx: usize) -> Option<char> {
    for (idi, chi) in ('A'..='Z').enumerate() {
        if idx == idi {
            return Some(chi);
        }
    }
    None
}

// Returns the matching character in the Vigenere matrix, depending
// on the header (ch_m) and column (ch_k) characters provided
fn vig_matcher(m: &VigMatrix, ch_m: char, ch_k: char) -> char {
    let idx_c = idx_finder(ch_m).unwrap();
    let idx_r = idx_finder(ch_k).unwrap();

    m[idx_r][idx_c]
}

// Prints a Vigenere matrix
fn print_vig_matrix(m: &VigMatrix) {
    for r in 0..m.len() {
        for c in 0..m.len() {
            print!("{} ", m[r][c]);
        }
        println!("");
    }
}

// Creates and returns a new Vigenere Matrix
fn new_vig_matrix() -> VigMatrix {
    let mut mat: VigMatrix = [[' '; 26]; 26];
    let mut acc = ('A'..='Z').cycle();

    for r in 0..=25 {
        for c in 0..=25 {
            mat[r][c] = acc.next().unwrap();
        }
        acc.next();
    }

    mat
}

fn complete_key(key: &str, msg_size: usize) -> String {
    let mut key_chars = key.chars().cycle();
    let mut new_key = "".to_string();
    for _ in 0..msg_size {
        new_key.push(key_chars.next().unwrap());
    }
    new_key
}

fn remove_spaces(s: &str) -> String {
    let sr: Vec<_> = s.trim().split_whitespace().collect();
    sr.join("")
}

fn encrypt(msg: &str, key: &str) -> String {
    // clean message and key
    let msg = &remove_spaces(msg);
    let key = &remove_spaces(key);

    // get size of message and key
    let msg_size = msg.chars().count();
    let key_size = key.chars().count();

    // initializations
    let mut encrypted_msg = "".to_string();
    let vig_mat = new_vig_matrix();

    // if key has a differnt size, then complete it
    let mut key_e = key.to_string();
    if msg_size != key_size {
        key_e = complete_key(key, msg_size);
    }

    // convert to char vectors
    let key_chars: Vec<_> = key_e.to_string().to_uppercase().chars().collect();
    let msg_chars: Vec<_> = msg.to_string().to_uppercase().chars().collect();

    // encrypt message
    for i in 0..msg_size {
        encrypted_msg.push(vig_matcher(&vig_mat, msg_chars[i], key_chars[i]));
    }

    encrypted_msg
}

pub(crate) fn decrypt(encr_msg: &str, key: &str) -> String {
    // clean message and key
    let encr_msg = &remove_spaces(encr_msg);
    let key = &remove_spaces(key);

    // get size of message and key
    let msg_size = encr_msg.chars().count();
    let key_size = key.chars().count();

    // initializations
    let mut decrypted_msg = "".to_string();
    let vig_mat = new_vig_matrix();

    // if key has a differnt size, then complete it
    let mut key_e = key.to_string();
    if msg_size != key_size {
        key_e = complete_key(key, msg_size);
    }

    // convert to char vectors
    let key_chars: Vec<_> = key_e.to_string().to_uppercase().chars().collect();
    let msg_chars: Vec<_> = encr_msg.to_string().to_uppercase().chars().collect();

    // decrypt message
    for letter in 0..msg_size {
        let mut msg_idx = 0;
        let key_idx = idx_finder(key_chars[letter]).unwrap();
        for c in 0..=25 {
            if vig_mat[key_idx][c] == msg_chars[letter] {
                msg_idx = c;
            }
        }
        decrypted_msg.push(char_finder(msg_idx).unwrap());
    }

    decrypted_msg
}

use std::fmt::Display;

pub struct Hello {
    name: String,
}
pub fn new_hello() -> Hello {
    Hello {
        name: "Rodrigo".to_string(),
    }
}
impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
