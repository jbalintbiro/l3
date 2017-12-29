#![feature(nll)]
#![feature(conservative_impl_trait)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::FromIterator;

macro_rules! import_submodules {
	($m:ident) => (mod $m; use $m::*;);
	($m:ident, $($ms:ident),+) => (import_submodules!($m); import_submodules!($($ms),+););
}

import_submodules!(value, func, parse, internals, eval, bindings, builtins, read);

fn main() {
	let root_bindings =	default_root();
	//let program = "(fn hello () (print (* 2 (+ 1 2)))) (hello) (print (quote 1 2 3)) (print (list (+ 1 1) (* 2 2)))";
	let program = "(print (* 2 (+ 1 2))) (print (quote 1 2 3)) (print (list (+ 1 1) (* 2 2)))";

	for term in read_program(program).iter() {
		eval(term, root_bindings.clone());
	}
}

#[cfg(test)]
mod tests;
