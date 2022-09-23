/*
Copyright 2022 Rodrigo Santiago.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
*/

use std::fmt::Display;

const SIZE: usize = 225;
type VigMatrix = [[char; SIZE]; SIZE];
pub(crate) struct DictWrap(pub(crate) Vec<char>);

#[derive(Debug)]
pub(crate) enum ErrorCode {
    InvalidChar(char),
    InvalidIndex(usize),
}

// Function to display the dictionary using a Wrapper
impl Display for DictWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = write!(f, "");
        for _ in 0..self.0.len() {
            s = write!(f, "{}", self.0.iter().next().unwrap());
        }
        return s;
    }
}

// Creates and returns a new dictionary for the Vigenere Matrix
pub(crate) fn new_dictionary() -> DictWrap {
    let mut dict: Vec<char> = Vec::with_capacity(SIZE);
    for i in 0..=255 {
        if !char::from_u32(i).unwrap().is_control() {
            dict.push(char::from_u32(i).unwrap());
        }
    }
    // push carriage return characters
    dict.push(char::from_u32(10).unwrap());
    dict.push(char::from_u32(13).unwrap());

    return DictWrap(dict);
}

// Creates and returns a new Vigenere Matrix
pub(crate) fn new_vig_matrix() -> VigMatrix {
    let mut mat: VigMatrix = [[' '; SIZE]; SIZE];
    // let mut acc = (' '..='~').cycle();
    let binding = new_dictionary().0;
    let mut acc = binding.iter().cycle();

    for r in 0..mat.len() {
        for c in 0..mat.len() {
            mat[r][c] = *acc.next().unwrap();
        }
        acc.next();
    }
    // print_vig_matrix(&mat);
    return mat;
}

// Returns the index value of a char in the Vigenere matrix
fn idx_finder(ch: char, m: &VigMatrix) -> Result<usize, ErrorCode> {
    for (idx, chi) in m[0].iter().enumerate() {
        if ch == *chi {
            return Ok(idx);
        }
    }
    Err(ErrorCode::InvalidChar(ch))
}

// Returns the char value of an index in the Vigenere matrix
fn char_finder(idx: usize, m: &VigMatrix) -> Result<char, ErrorCode> {
    for (idi, chi) in m[0].iter().enumerate() {
        if idx == idi {
            return Ok(*chi);
        }
    }
    Err(ErrorCode::InvalidIndex(idx))
}

// Returns the matching character in the Vigenere matrix, depending
// on the header (ch_m) and column (ch_k) characters provided
fn vig_matcher(m: &VigMatrix, ch_m: char, ch_k: char) -> Result<char, ErrorCode> {
    let idx_c = idx_finder(ch_m, &m)?;
    let idx_r = idx_finder(ch_k, &m)?;

    Ok(m[idx_r][idx_c])
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

fn complete_key(key: &str, msg_size: usize) -> String {
    let mut key_chars = key.chars().cycle();
    let mut new_key = "".to_string();
    for _ in 0..msg_size {
        new_key.push(key_chars.next().unwrap());
    }
    new_key
}

pub(crate) fn encrypt(msg: &str, key: &str) -> Result<String, ErrorCode> {
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
    let key_chars: Vec<_> = key_e.to_string().chars().collect();
    let msg_chars: Vec<_> = msg.to_string().chars().collect();

    // encrypt message
    for i in 0..msg_size {
        encrypted_msg.push(vig_matcher(&vig_mat, msg_chars[i], key_chars[i])?);
    }

    Ok(encrypted_msg)
}

pub(crate) fn decrypt(encr_msg: &str, key: &str) -> Result<String, ErrorCode> {
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
    let key_chars: Vec<_> = key_e.to_string().chars().collect();
    let msg_chars: Vec<_> = encr_msg.to_string().chars().collect();

    // decrypt message
    for letter in 0..msg_size {
        let mut msg_idx = 0;
        let mut key_idx = idx_finder(key_chars[letter], &vig_mat)?;
        for c in 0..vig_mat.len() {
            if vig_mat[key_idx][c] == msg_chars[letter] {
                msg_idx = c;
            }
        }
        decrypted_msg.push(char_finder(msg_idx, &vig_mat)?);
    }

    Ok(decrypted_msg)
}
