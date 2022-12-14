#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::pedantic,
    clippy::missing_docs_in_private_items,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::clone_on_ref_ptr,
    clippy::integer_division,
    clippy::lossy_float_literal,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_inherent_impl,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented
)]
#![deny(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::panic_in_result_fn,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::try_err,
    clippy::same_name_method,
    clippy::unneeded_wildcard_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result,
)]
#![allow(clippy::module_name_repetitions)]

//! Allows you to get requests from guilded over a websocket connection.

pub mod events;
pub mod client;

pub use client::connect_to_websocket;