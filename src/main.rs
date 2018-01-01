#![feature(nll)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate gc_derive;
extern crate gc;

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
	let root_bindings = default_root();
	for term in read_program_file(infile).iter() {
		eval(term, root_bindings.clone());
	}
}

#[cfg(test)]
mod tests;
