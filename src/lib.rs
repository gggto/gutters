//! This library provides very *very* basic generic functions for building
//! quick and dirty interprocess or network protocols. Sewer metaphors
//! included.
//!
//! # Usage
//!
//! ```rust
//! use gutters::{pick_up, throw, hail, wait, throw_and_wait, pick_up_and_hail};
//!
//! // Here we use a TcpStream as a "gutter", but anything implementing
//! // Read and Write will do.
//! use std::net::TcpStream;
//! let mut stream = TcpStream::connect("127.0.0.1:34254");
//! // Of course, here you need a server on the other side.
//!
//! // You can "throw" data down the "gutter" with the corresponding
//! // function. The other side will have to "pick-up" the corresponding
//! // message (or "log", if you want to pursue the metaphor).
//! let log = 123.4f64
//! throw(&mut stream, &log)?;
//!
//! // You can "throw" as many "logs" as you want, while the other end
//! // "picks them up".
//! let log = 567.8f64
//! throw(&mut stream, &log)?;
//! throw(&mut stream, &log)?;
//!
//! // You can also "pick-up logs".
//! let mut log = 0.0f64;
//! pick_up(&mut stream, &mut log)?;
//! println!("{}", log)?;
//! pick_up(&mut stream, &mut log)?;
//! println!("{}", log)?;
//!
//! // If you need to synchronize with the other end, you can "hail" to
//! // them. They will have to "wait" for you.
//! hail(&mut stream)?;
//!
//! // You can also "wait" for the other end to "hail" you.
//! wait(&mut stream)?;
//!
//! // If you want to be synchronized with the other end at all time,
//! // you may use the `pick_up_and_hail` and `throw_and_wait` variants.
//! let log = 567.8f64
//! throw_and_wait(&mut stream, &log)?;
//!
//! let mut log = 0.0f64;
//! pick_up_and_hail(&mut stream, &mut log)?;
//! println!("{}", log)?;
//! ```

use std::io::{Read, Result, Write};

fn as_u8_slice_mut<T>(v: &mut T) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut((v as *mut T) as *mut u8, std::mem::size_of::<T>()) }
}

fn as_u8_slice<T>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((v as *const T) as *const u8, std::mem::size_of::<T>()) }
}

/// Read a message of type `T` from the `gutter`.
///
/// This function is blocking.
///
/// This function doesn't change endianness, so it must be the
/// same between the peers.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::net::TcpStream;
/// let stream = TcpStream::connect("127.0.0.1:34567")
///
/// let mut data = 0.0f64
/// pick_up(&mut stream, &mut data)?;
/// ```
pub fn pick_up<G: Read + Write, T>(gutter: &mut G, buffer: &mut T) -> Result<()> {
    gutter.read_exact(as_u8_slice_mut(buffer))
}

/// Send a message of type `T` to the `gutter`.
///
/// This function is blocking.
///
/// This function doesn't change endianness, so it must be the
/// same between the peers.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::net::TcpStream;
/// let stream = TcpStream::connect("127.0.0.1:34567")
///
/// throw(&mut stream, &64.0)?;
/// ```
pub fn throw<G: Read + Write, T>(gutter: &mut G, buffer: &T) -> Result<()> {
    gutter.write_all(as_u8_slice(buffer))
}

/// Send an acknowledgment to the `gutter`.
///
/// This function is blocking.
///
/// The acknowledgment is a single byte `b'\n'`.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::net::TcpStream;
/// let stream = TcpStream::connect("127.0.0.1:34567")
///
/// hail(&mut stream)?;
/// ```
pub fn hail<G: Write>(gutter: &mut G) -> Result<()> {
    gutter.write_all(b"\n")
}

/// Wait for an acknowledgment from the `gutter`.
///
/// This function is blocking.
///
/// The exact byte value of the acknowledgment is *not* checked for.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::net::TcpStream;
/// let stream = TcpStream::connect("127.0.0.1:34567")
///
/// wait(&mut stream)?;
/// ```
pub fn wait<G: Read>(gutter: &mut G) -> Result<()> {
    gutter.read_exact(&mut [0u8])
}

/// Read a message of type `T` from the `gutter`, and send an
/// acknowledgement.
///
/// Equivalent to calling [`pick_up`] and then [`hail`].
///
/// This function is blocking.
///
/// This function doesn't change endianness, so it must be the
/// same between the peers.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::net::TcpStream;
/// let stream = TcpStream::connect("127.0.0.1:34567")
///
/// let mut data = 0.0f64
/// pick_up_and_hail(&mut stream, &mut data)?;
/// ```
pub fn pick_up_and_hail<G: Read + Write, T>(gutter: &mut G, buffer: &mut T) -> Result<()> {
    pick_up(gutter, buffer)?;
    hail(gutter)
}

/// Send a message of type `T` to the `gutter`, and wait for an
/// acknowledgement.
///
/// Equivalent to calling [`throw`] and then [`wait`].
///
/// This function is blocking.
///
/// This function doesn't change endianness, so it must be the
/// same between the peers.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use std::net::TcpStream;
/// let stream = TcpStream::connect("127.0.0.1:34567")
///
/// throw(&mut stream, &64.0)?;
/// ```
pub fn throw_and_wait<G: Read + Write, T>(gutter: &mut G, buffer: &T) -> Result<()> {
    throw(gutter, buffer)?;
    wait(gutter)
}
