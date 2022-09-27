# Gutters

***Quick and dirty tools for the intrepid plumber***

`gutters` provides very *very* basic generic functions for building
quick and dirty interprocess or network protocols. Sewer metaphors
included.

Python bindings are available [here](https://github.com/gggto/pygutters).

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
gutters = "0.1.0"
```

Using the library is quite simple:
```rust
use gutters::{pick_up, throw, hail, wait, throw_and_wait, pick_up_and_hail};

// Here we use a TcpStream as a "gutter", but anything implementing
// Read and Write will do.
use std::net::TcpStream;
let mut stream = TcpStream::connect("127.0.0.1:34254");
// Of course, here you need a server on the other side.

// You can "throw" data down the "gutter" with the corresponding
// function. The other side will have to "pick-up" the corresponding
// message (or "log", if you want to pursue the metaphor).
let log = 123.4f64
throw(&mut stream, &log)?;

// You can "throw" as many "logs" as you want, while the other end
// "picks them up".
let log = 567.8f64
throw(&mut stream, &log)?;
throw(&mut stream, &log)?;

// You can also "pick-up logs".
let mut log = 0.0f64;
pick_up(&mut stream, &mut log)?;
println!("{}", log)?;
pick_up(&mut stream, &mut log)?;
println!("{}", log)?;

// If you need to synchronize with the other end, you can "hail" to
// them. They will have to "wait" for you.
hail(&mut stream)?;

// You can also "wait" for the other end to "hail" you.
wait(&mut stream)?;

// If you want to be synchronized with the other end at all time,
// you may use the `pick_up_and_hail` and `throw_and_wait` variants.
let log = 567.8f64
throw_and_wait(&mut stream, &log)?;

let mut log = 0.0f64;
pick_up_and_hail(&mut stream, &mut log)?;
println!("{}", log)?;
```