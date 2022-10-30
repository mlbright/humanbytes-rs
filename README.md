# humanize-rs

> Format byte sizes into more humanly readable values

This program was inspired by a [post][wandering-thoughts] by Chris Siebenmann.
I wondered if there was an equivalent (and easy) way to perform the functions of his Python script in Rust.

There is!
This is one possible solution:

## Run

    cargo run --release --example bench < samples/test.txt

[wandering-thoughts]: https://utcc.utoronto.ca/~cks/space/blog/python/RegexpFunctionSubstitutionWin?showcomments#comments
