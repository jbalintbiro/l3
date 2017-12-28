extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::inputs::{Input, Span};
use pest::iterators::Pair;


#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("l3.pest"); // relative to this file

#[derive(Parser)]
#[grammar = "l3.pest"]
struct L3Parser;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

fn pair(head: Value, tail: List) -> Value {
	Value::List(
		List::Cons(
			Rc::new(
				(
					RefCell::new(head),
					RefCell::new(tail)
				)
			),
		)
	)
}

#[derive(Debug)]
pub enum List {
	Nil,
	Cons(Rc<(RefCell<Value>, RefCell<List>)>),
}

fn print_list_inner(inner: &(RefCell<Value>, RefCell<List>), f: &mut fmt::Formatter) -> fmt::Result {
	let (ref h, ref t) = *inner;
	write!(f, "{}", h.borrow())?;
	match *t.borrow() {
		List::Nil => {},
		List::Cons(ref i) => {
			write!(f, " ")?;
			print_list_inner(i, f)?
		}
	}
	write!(f, "")
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(")?;
		match *self {
			List::Nil => {},
			List::Cons(ref inner) => {
				print_list_inner(&*inner, f)?;
			}
		};
		write!(f, ")")
    }
}


#[derive(Debug)]
pub enum Value {
	List(List),
	False,
	True,
	Int(i32),
	Ident(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Value::List(ref l) => write!(f, "{}", l),
			Value::False => write!(f, "false"),
			Value::True => write!(f, "true"),
			Value::Int(ref i) => write!(f, "{}", i),
			Value::Ident(ref s) => write!(f, "{}", s),
		}
    }
}

struct ListBuilder {
	head: Option<List>,
	cur: List,
}

impl ListBuilder {
	fn new() -> ListBuilder {
		ListBuilder {
			head: None,
			cur: List::Nil,
		}
	}

	fn build(self) -> List {
		match self.head {
			None => List::Nil,
			Some(ret) => ret,
		}
	}

	fn push(&mut self, v: Value) {
		let next = Rc::new((RefCell::new(v), RefCell::new(List::Nil)));
		if self.head.is_none() {
			self.head = Some(List::Cons(next.clone()));
			self.cur = List::Cons(next);
		} else {
			let new;
			if let List::Cons(ref c) = self.cur {
				*c.1.borrow_mut() = List::Cons(next.clone());
				new = List::Cons(next);
			} else {
				unreachable!();
			}
			self.cur = new;
		}
	}
}


fn truthy(v: Value) -> bool {
	match v {
		Value::List(List::Nil) | Value::False => false,
		_ => true,
	}
}

fn parse_program<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
	for program in pairs {
		match program.as_rule() {
			Rule::program => return parse(program.into_inner()),
			v => { panic!("program not a program {:?}", v); }
		};
	};
	unreachable!()
}

fn parse_list<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
	let mut head: Option<List> = None;
	let mut cur: List = List::Nil;
	for pair in pairs {
		match pair.as_rule() {
			Rule::term => {
				let next = Rc::new((RefCell::new(parse(pair.into_inner())), RefCell::new(List::Nil)));
				if head.is_none() {
					head = Some(List::Cons(next.clone()));
					cur = List::Cons(next);
				} else {
					if let List::Cons(c) = cur {
						*c.1.borrow_mut() = List::Cons(next.clone());
						cur = List::Cons(next);
					} else {
						unreachable!();
					}
				}
			}
			_ => {},
		}
	}

	match head {
		None => Value::List(List::Nil),
		Some(ret) => Value::List(ret),
	}
}

fn parse<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
	for pair in pairs {
		match pair.as_rule() {
			Rule::term => {
				return parse(pair.into_inner())
			}
			Rule::list => {
				return parse_list(pair.into_inner())
			},
			Rule::integer => return Value::Int(pair.into_span().as_str().parse().unwrap()),
			Rule::ident => return Value::Ident(String::from(pair.into_span().as_str())),
			v => panic!("wtf: {:?}", v),
		}
	}
	unreachable!()
}

fn read(inp: &str) -> Value {
	println!("{}", inp);
	let pairs = L3Parser::parse_str(Rule::program, inp).unwrap_or_else(|e| panic!("{}", e));
	println!("PARSED: {}", parse_program(pairs));
	println!("");
	Value::False
}

fn main() {
	read("(a) (1 2 3)");
	read("(1 2 3 (foo bar))");
	read("(1 2 3 ;;lol \n 4)");
	read("(() () () no)");
	//use Value::*;
//	println!("{}", truthy(pair(Int(1), Nil)));
//	println!("{}", truthy(Nil));
}
