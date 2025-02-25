#![feature(
	concat_idents,
	proc_macro_hygiene
)]
#![allow(
	unused_macros,
	clippy::borrow_interior_mutable_const
)]
    
mod edge;

#[skyline::main(name = "edge")]
pub fn main() {
	edge::install();
}