#![feature(nll)]
#![feature(conservative_impl_trait)]

#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

#[macro_use]
extern crate clap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate gc;
#[macro_use]
extern crate gc_derive;

#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
use pest::Parser;

use std::fmt;
use std::rc::Rc;
use std::iter::FromIterator;

use clap::{Arg, App};

use gc::{Gc, GcCell};

macro_rules! import_submodules {
	($m:ident) => (mod $m; use $m::*;);
	($m:ident, $($ms:ident),+) => (import_submodules!($m); import_submodules!($($ms),+););
}

import_submodules!(value, func, parse, internals, eval, bindings, builtins, read);

const PRELUDE: &'static str = include_str!("prelude.l3");

pub fn loaded_env() -> LCell<Bindings> {
	let root_bindings = default_root();
	for term in read_program(PRELUDE).iter() {
		eval(term, root_bindings.clone());
	}
	root_bindings
}

pub fn run_program(program: LCell<Value>, env: LCell<Bindings>) {
	for term in program.borrow().iter() {
		eval(term, env.clone());
	}
}

fn main() {
	let opts = App::new(crate_name!())
					.version(crate_version!())
					.author(crate_authors!("\n"))
					.about(crate_description!())
					.arg(Arg::with_name("INPUT")
						.help("path to L3 program to interpret")
						.required(true))
					.get_matches();

	let infile = opts.value_of("INPUT").unwrap();
	run_program(lcell(read_program_file(infile)), loaded_env());
}

#[cfg(test)]
mod tests;
