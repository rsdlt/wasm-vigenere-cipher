mod cipher;
use cipher::Hello;
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let hello_r = cipher::new_hello();

    let displayed_name = || {
        if name.get().is_empty() {
            "".to_string()
        } else {
            name.get().as_ref().clone()
        }
    };

    view! { cx,
        div {
            h2 {
                "Real-Time Vigénere Cipher"
                // "! "
                // (hello_r)
            }
            p { input(placeholder="Enter a phrase", bind:value=name) }
            p { strong{"Key: "} (displayed_name())}
            p { strong{"Encrypted: "} (displayed_name())}
            p { strong{"Decrypted: "} (displayed_name())}
        }
    }
}

fn main() {
    // console_error_panic_hook::set_once();
    // console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
