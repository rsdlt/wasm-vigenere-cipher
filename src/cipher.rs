/*
Copyright 2022 Rodrigo Santiago.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
*/

pub(crate) const SIZE: usize = 225;

#[derive(Clone, Copy)]
pub(crate) struct VigMatrixWrap(pub(crate) [[char; SIZE]; SIZE]);

#[derive(Clone, Copy)]
pub(crate) struct DictWrap(pub(crate) [char; SIZE]);

#[derive(Debug)]
pub(crate) enum ErrorCode {
    InvalidChar(char),
    InvalidIndex(usize),
}

// Creates and returns a new dictionary for the Vigenere Matrix.
impl DictWrap {
    pub(crate) fn new() -> DictWrap {
        // Every ASCII character that !is_control().
        let mut dict = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##.to_string();
        // Add carriage return to support in web textarea.
        dict.push('\n');
        dict.push('\r');
        let mut dict_char_arr = [' '; SIZE];
        for (idx, ch) in dict.chars().enumerate() {
            dict_char_arr[idx] = ch;
        }
        return DictWrap(dict_char_arr);
    }

    pub(crate) fn get_string(&self) -> String {
        let mut s = String::new();
        for ch in self.0 {
            s.push(ch);
        }
        s
    }
}
// Creates and returns a new Vigenere Matrix.
impl VigMatrixWrap {
    pub(crate) fn new() -> VigMatrixWrap {
        let mut mat: VigMatrixWrap = VigMatrixWrap([[' '; SIZE]; SIZE]);
        let binding = DictWrap::new().0;
        let mut acc = binding.iter().cycle();

        for r in 0..mat.0.len() {
            for c in 0..mat.0.len() {
                mat.0[r][c] = *acc.next().unwrap();
            }
            acc.next();
        }
        return mat;
    }
}

// Returns the index value of a char in the Vigenere matrix.
fn idx_finder(ch: char, m: &VigMatrixWrap) -> Result<usize, ErrorCode> {
    for (idx, chi) in m.0[0].iter().enumerate() {
        if ch == *chi {
            return Ok(idx);
        }
    }
    Err(ErrorCode::InvalidChar(ch))
}

// Returns the char value of an index in the Vigenere matrix
fn char_finder(idx: usize, m: &VigMatrixWrap) -> Result<char, ErrorCode> {
    for (idi, chi) in m.0[0].iter().enumerate() {
        if idx == idi {
            return Ok(*chi);
        }
    }
    Err(ErrorCode::InvalidIndex(idx))
}

// Returns the matching character in the Vigenere matrix, depending
// on the header (ch_m) and column (ch_k) characters provided
fn vig_matcher(m: &VigMatrixWrap, ch_m: char, ch_k: char) -> Result<char, ErrorCode> {
    let idx_c = idx_finder(ch_m, &m)?;
    let idx_r = idx_finder(ch_k, &m)?;

    Ok(m.0[idx_r][idx_c])
}

// Completes the key if the size is not the same as the message.
fn complete_key(key: &str, msg_size: usize) -> String {
    let mut key_chars = key.chars().cycle();
    let mut new_key = "".to_string();
    for _ in 0..msg_size {
        new_key.push(key_chars.next().unwrap());
    }
    new_key
}

// Encodes a message (msg) with a key (key) using a Vigenere Matix (vig_mat).
pub(crate) fn encode(msg: &str, key: &str, vig_mat: VigMatrixWrap) -> Result<String, ErrorCode> {
    // get size of message and key
    let msg_size = msg.chars().count();
    let key_size = key.chars().count();

    // initializations
    let mut encrypted_msg = "".to_string();
    // let vig_mat = VigMatrixWrap::new();

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

// Decodes an encoded message (enc_msg) with a key (key) and a Vigenere Matrix (vig_mat).
pub(crate) fn decode(
    enc_msg: &str,
    key: &str,
    vig_mat: VigMatrixWrap,
) -> Result<String, ErrorCode> {
    // get size of message and key
    let msg_size = enc_msg.chars().count();
    let key_size = key.chars().count();

    // initializations
    let mut decrypted_msg = "".to_string();

    // if key has a differnt size, then complete it
    let mut key_e = key.to_string();
    if msg_size != key_size {
        key_e = complete_key(key, msg_size);
    }

    // convert to char vectors
    let key_chars: Vec<_> = key_e.to_string().chars().collect();
    let msg_chars: Vec<_> = enc_msg.to_string().chars().collect();

    // decrypt message
    for letter in 0..msg_size {
        let mut msg_idx = 0;
        let key_idx = idx_finder(key_chars[letter], &vig_mat)?;
        for c in 0..vig_mat.0.len() {
            if vig_mat.0[key_idx][c] == msg_chars[letter] {
                msg_idx = c;
            }
        }
        decrypted_msg.push(char_finder(msg_idx, &vig_mat)?);
    }

    Ok(decrypted_msg)
}
