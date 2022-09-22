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
    let key = "RUST IS COOL";

    let phrase = create_signal(cx, String::new());
    let encr_signal = create_signal(cx, String::new());
    let decr_signal = create_signal(cx, String::new());

    let disp_phrase = || {
        if phrase.get().is_empty() {
            encr_signal.set("".to_string());
            decr_signal.set("".to_string());
            "".to_string()
        } else {
            encr_signal.set(encrypt(&phrase.get().as_ref().clone(), key));
            decr_signal.set(decrypt(&encr_signal.get().as_ref().clone(), key));
            phrase.get().as_ref().clone()
        }
    };
    let disp_encr = || {
        if encr_signal.get().is_empty() {
            "not encrypted".to_string()
        } else {
            encr_signal.get().as_ref().clone()
        }
    };
    let disp_decr = || {
        if decr_signal.get().is_empty() {
            "not decrypted".to_string()
        } else {
            decr_signal.get().as_ref().clone()
        }
    };

    view! { cx,
        div {
            h2 { "Real-Time Vigénere Cipher" }

            p { input(placeholder="Enter a phrase", bind:value=phrase) }

            p { strong{"Key: "} (key) }

            p { strong{"Encrypted: "} (disp_encr()) }
            p { strong{"Decrypted: "} (disp_decr()) }
            p { strong{"Phrase: "} (disp_phrase()) }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
