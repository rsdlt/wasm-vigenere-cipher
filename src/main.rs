/*
Copyright 2022 Rodrigo Santiago.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.

Sycamore Library License:
The MIT License (MIT)
Copyright © 2021-2022 Luke Chu

*/

#![allow(unused)]

mod cipher;

use cipher::*;
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let key = "°¡! RüST íS CóÓL ¡!°";
    let dict = DictWrap::new().get_string();

    // Signals declaration.
    let phrase = create_signal(cx, String::new());
    let encr_signal = create_signal(cx, String::new());
    let decr_signal = create_signal(cx, String::new());
    let warning_signal = create_signal(cx, String::new());
    let dict_signal = create_signal(cx, String::new());
    let mat_signal = create_signal(cx, VigMatrixWrap::new());

    // Memo declaration tied to phrase update in the textarea.
    let phrase_update = create_memo(cx, move || {
        if phrase.get().is_empty() {
            encr_signal.set("".to_string());
            decr_signal.set("".to_string());
            warning_signal.set("".to_string());
            dict_signal.set(dict.clone());
            // vigmat_signal.set(vigmat_web_render.clone());
        } else {
            match encode(
                &phrase.get().as_ref().clone(),
                key,
                mat_signal.get().as_ref().clone(),
            ) {
                Ok(ok_phrase) => {
                    encr_signal.set(ok_phrase);
                    warning_signal.set("".to_string());
                }
                Err(error_kind) => match error_kind {
                    ErrorCode::InvalidChar(ic) => {
                        warning_signal.set(format!("Invalid character: {}", ic))
                    }
                    ErrorCode::InvalidIndex(ii) => {
                        warning_signal.set(format!("Invalid index: {}", ii))
                    }
                },
            }
            match decode_web(
                &encr_signal.get().as_ref().clone(),
                key,
                mat_signal.get().as_ref().clone(),
            ) {
                Ok(ok_phrase) => {
                    decr_signal.set(ok_phrase);
                }
                Err(error_kind) => match error_kind {
                    ErrorCode::InvalidChar(ic) => {
                        warning_signal.set(format!("Invalid character: {}", ic))
                    }
                    ErrorCode::InvalidIndex(ii) => {
                        warning_signal.set(format!("Invalid index: {}", ii))
                    }
                },
            }
        }
    });

    let disp_dict = || {
        if dict_signal.get().is_empty() {
            "".to_string()
        } else {
            dict_signal.get().as_ref().clone()
        }
    };
    let disp_encr = || {
        if encr_signal.get().is_empty() {
            "".to_string()
        } else {
            encr_signal.get().as_ref().clone()
        }
    };
    let disp_decr = || {
        if decr_signal.get().is_empty() {
            "".to_string()
        } else {
            decr_signal.get().as_ref().clone()
        }
    };
    let disp_warning = || {
        if warning_signal.get().is_empty() {
            "".to_string()
        } else {
            warning_signal.get().as_ref().clone()
        }
    };

    view! { cx,
            div {
                h1 { "Real-Time Vigénere Cipher" }

                p { strong{"Key: "} "[" span(style="color:Tomato; font-family:'Courier New';"){(key)} "]" }

                p { textarea(placeholder="Enter a phrase...", autofocus=true, maxlength="50000", bind:value=phrase) }
                p { span(style="color:Tomato"){(disp_warning())}}

                p { strong{"Encoded: "} br{}
                    "[" span(style="color:Tomato; font-family:'Courier New';"){(disp_encr())} "]" }
                p { strong{"Decoded: "} br{}
                    "[" span(style="color:MediumSeaGreen; font-family:'Courier New';"){span(dangerously_set_inner_html=&(disp_decr()))} "]" }

    // div(dangerously_set_inner_html=&(disp_vigmat()))
                p { "The encoding dictionary includes the following set of " (SIZE) " ASCII characters:" br{}
                  "[" span(style="color:Orchid;font-family:'Courier';"){(disp_dict())} "]" }

                footer {
                    small{"Copyright 2022, " a(href="https://rsdlt.github.io/about/"){"Rodrigo Santiago"} ", " a(href="https://rsdlt.github.io/about/#terms-of-use"){"Terms of use"}}
                   p { a(href="https://github.com/rsdlt/wasm-vigenere-cipher"){"GitHub Repo"} }
                }
            }
        }
}

fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
