//! CodeSnap is a tool to generate beautiful snapshots of your code snippets. It's a pure Rust library
//! that provides a simple API to create snapshots of code snippets with syntax highlighting, line
//! numbers, and custom themes.
//!
//! # Quick start
//!
//! ```rust
//! CodeSnap::default()
//!   .code(
//!       CodeBuilder::default()
//!           .language("rust")
//!           .content(CODE_SNIPPET)
//!           .build()?,
//!   )
//!   .watermark(WatermarkBuilder::default().content("CodeSnap").build()?)
//!   .build()?
//!   .create_snapshot()?
//!   .copy()
//! ```
//!
//! Now try to paste the code snapshot to your friends!
//!
//!  /\_/\
//! ( -.- )  If you enjoy CodeSnap, also try [Silicon](https://github.com/Aloxaf/silicon)
//!

mod components;
pub mod config;
pub mod edges;
pub mod preset_background;
pub mod snapshot;
pub mod utils;
