# WebAssembly Vigenère Cipher

A simple [Rust] and [WebAssembly] real-time implementation of the [Vigenère Cipher] utilizing the [Sycamore] reactive library, [Trunk] for build & bundle, and [Water.css] for styling.

Check out the [demo here].

## Features:
- The cipher supports 192 ASCII characters, all non-control plus `'\n'` and `'\r'`.
- Handles non-supported chars gracefully.
- Properly displays multiple continuous space characters and carriage return / new line.

![features](/img/demo-3.gif)

![features](/img/demo-2.gif)

[WebAssembly]:https://webassembly.org/
[Sycamore]:https://sycamore-rs.netlify.app/
[Rust]:https://www.rust-lang.org
[Vigenère Cipher]:https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
[demo here]:https://wasm-vigenere-cipher.onrender.com/
[Water.css]:https://watercss.kognise.dev/
[Trunk]:https://trunkrs.dev/
