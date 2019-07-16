[![MIT][s2]][l2] [![Chat on Miaou][s4]][l4]

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

# wasm-tictactoe
Example of a 100% pure Rust, framework free, Wasm application

I've written about the experience in dev.to: [An exploration of Rust/Wasm](https://dev.to/dystroy/an-exploration-or-rust-wasm-50gp)

## Goal

The goal is to answer this simple question: *How convenient and efficient is it to build a 100% Rust Wasm Web application?*

To support this exploration I build a simple Tic-Tac-Toe.

**[Test it here](https://dystroy.org/wasm-tictactoe/)**

I especially want it

- clean and understandable
- non leaking
- free of unsafe
- compile time checked, with the guarantees of a typical rust app
- long running app
- to demonstrate DOM access, callbacks
- no lib or framework other than js_sys and web_sys
- to avoid crazy tool chains with npm, webpack, parcels, etc.
- using all the facilities we love in Rust

and on top of that I want to

- check file sizes
- check performances

This is still a Work In Progress.

## How to build

Other than the usual rust/cargo toolchain, you need [wasm-pack](https://github.com/rustwasm/wasm-pack).

The application is built with

	wasm-pack build --target web

The files you need to deploy afterwards are

- index.html
- tictactoe.css
- pkg/wasm_tictactoe.js : a boostraper loading your wasm file
- pkg/wasm_tictactoe_bg.wasm : your compiled wasm


## Main Findings

| Concern | Reached | Comments |
|:-:|:-:|:-|
| clean and understandable | kind of | It's globally OK (meaning it could be OK with just a little work and some documentation) |
| non leaking | seems OK | It takes a lot of work to do that, mainly because it's hard to keep around event handlers built in Rust without leaking them. |
| free of unsafe | no | Right now the cleanest solution I found to avoid leaking event handlers was to keep them in a mutable static holder wich involved some `unsafe`. Note that JS event handlers can't be send over threads which defeated my tentatives to use Arc Mutex for that|
| compile time checked | partly | To start with, not available `std` feature are always detected at runtime with a stacktrace ending in an `unreachable!()`. It's painful when you try to see whether you can use for example *mpsc* (unfortunately you can't). Then, there's the problem of the DOM API with functions returning variable type objects, which may lead to runtime failures (those should be easy to isolate, though). Globally It's still nowhere near the JavaScript experience and when it compiles it might be working.|
| long running app | ok | With some unsafe, this should be OK |
| DOM | OK | A tiny helper lib would help make it easier or friendler but there's everything in web_sys |
| callbacks | bad. Working but bad | You need a lot of weird and ugly code to deal with callbacks when you do everything from rust. And there's a lot of gymnastic if you don't want them to leak. This is the main pain point, the one which prevents any reasonnable work without a lib or framework (I don't know what exists) |
| no lib or framework | ok... sort of | My application proves it's possible to limit oneself to js_sys and web_sys. Some libs will probably be necessary. I miss channels...|
| tool chain | ok | I wrote all files by hand, and I'm just using `wasm-pack` to compile the wasm file, and it's not slower than your usual Rust compilation |
| Rust facilities | partly | Most of it is here, but we're lacking many Rust libs, sometimes for obvious reasons |
| File Size | ok | I was afraid. This was a good surprise. You can check the example online |
| Performances | ? | I still have no idea. Let's admit a tic-tac-toe isn't the best application to stress performances |


Now... All this was made with little Rust experience and absolutely no prealable wasm knowledge. It's certain there's room for improvements and I'd gladly accept remarks.
