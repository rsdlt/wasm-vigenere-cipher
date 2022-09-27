# WebAssembly Vigénere Cipher

A simple [Rust] and [WebAssembly] real-time implementation of the [Vigénere Cipher] utilizing the [Sycamore] reactive library, [Trunk] for build & bundle, and [Water.css] for styling.

Check out the [demo here].

The cipher supports 225 ASCII characters, all non-control plus `'\n'` and `'\r'`.

![char support](/img/demo-1.gif)

And handles non-supported chars gracefully:

![error handling](/img/demo-2.gif)

[WebAssembly]:https://webassembly.org/
[Sycamore]:https://sycamore-rs.netlify.app/
[Rust]:https://www.rust-lang.org
[Vigénere Cipher]:https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
[demo here]:https://wasm-vigenere-cipher.onrender.com/
[Water.css]:https://watercss.kognise.dev/
[Trunk]:https://trunkrs.dev/
