//! # Serde LTSV
//!
//! Labeled Tab-separated Values (LTSV) format is a variant of Tab-separated Values (TSV). Each
//! record in a LTSV file is represented as a single line. Each field is separated by TAB and has
//! a label and a value. The label and the value have been separated by ':'. With the LTSV format,
//! you can parse each line by spliting with TAB (like original TSV format) easily, and extend any
//! fields with unique labels in no particular order.
//!
//! ```
//! time:[10/Oct/2000:13:55:36 -0700]\tdone:true\tscore:-1\tmean:0.42\tcounter:42\tlevel:3\thost:testhostname\tname1:value1\tname 2: value 2\tn3:v3\tmessage:this is a test
//! ```
//!
//! ## Quickstart
//!
//! You can start using it by first adding it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde_derive = "1.0"
//! serde_ltsv = "0.1"
//! ```
//!
//! Then, create a structure which implement `serde::Serialize` / `serde::Deserialize` traits and
//! use the structure with any serde lib.
//!
//! ```rust
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_ltsv;
//! extern crate serde_json;
//! extern crate serde_yaml;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!     a: String,
//!     b: i8,
//!     c: bool
//! }
//!
//! fn main() {
//!     let t = Foo { a: "toto".into(), b: 8, c: true };
//!     let str_t = serde_ltsv::to_string(&t).unwrap();
//!     println!("As ltsv => {}", &str_t);
//!     let t2: Foo = serde_ltsv::from_str(&str_t).unwrap();
//!     println!("As json => {}", serde_json::to_string_pretty(&t2).unwrap());
//!     println!("As yaml => {}", serde_yaml::to_string(&t2).unwrap());
//! }
//! ```
//! **Output**:
//! ```text
//! As ltsv => a:toto	b:8	c:true
//! As json => {
//!     "a": "toto",
//!     "b": 8,
//!     "c": true
//! }
//! As yaml => ---
//! a: toto
//! b: 8
//! c: true
//! ```
extern crate serde;
extern crate serde_value;

pub use de::from_str;
pub use ser::to_string;
pub use result::LtsvError;

mod ser;
mod de;
mod result;

