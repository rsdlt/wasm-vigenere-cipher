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

    let phrase = create_signal(cx, String::new());
    let encr_signal = create_signal(cx, String::new());
    let decr_signal = create_signal(cx, String::new());
    let warning = create_signal(cx, String::new());

    let disp_phrase = || {
        if phrase.get().is_empty() {
            encr_signal.set("".to_string());
            decr_signal.set("".to_string());
            warning.set("".to_string());
            "".to_string()
        } else {
            match encrypt(&phrase.get().as_ref().clone(), key) {
                Ok(ok_phrase) => encr_signal.set(ok_phrase),
                Err(error_kind) => match error_kind {
                    ErrorCode::InvalidChar(ic) => warning.set(format!("Invalid character: {}", ic)),
                    ErrorCode::InvalidIndex(ii) => warning.set(format!("Invalid index: {}", ii)),
                },
            }
            match decrypt(&encr_signal.get().as_ref().clone(), key) {
                Ok(ok_phrase) => decr_signal.set(ok_phrase),
                Err(error_kind) => match error_kind {
                    ErrorCode::InvalidChar(ic) => warning.set(format!("Invalid character: {}", ic)),
                    ErrorCode::InvalidIndex(ii) => warning.set(format!("Invalid index: {}", ii)),
                },
            }
            phrase.get().as_ref().clone()
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
        if warning.get().is_empty() {
            "".to_string()
        } else {
            warning.get().as_ref().clone()
        }
    };

    view! { cx,
        div {
            h1 { "Real-Time Vigénere Cipher" }

            p { strong{"Key: "} "[" span(style="color:Tomato; font-family:'Courier New'"){(key)} "]" }

            p { input(placeholder="Enter a phrase", size="80", bind:value=phrase) }

            p { small{"Original: " (disp_phrase())} }

            p { strong{"Encoded: "} "[" span(style="color:Tomato; font-family:'Courier New'"){(disp_encr())} "]" }
            p { strong{"Decoded: "} "[" span(style="color:MediumSeaGreen;"){(disp_decr())} "]" }

            p { span(style="color:Tomato"){(disp_warning())}}

            footer {
               p {"Copyright 2022, " a(href="https://rsdlt.github.io/about/"){"Rodrigo Santiago"}}
               p { a(href="https://rsdlt.github.io/about/#terms-of-use"){"Terms of use and license"} }
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
