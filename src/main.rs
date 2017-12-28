#![feature(nll)]

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

type LCell<T> = Rc<RefCell<T>>;

fn lcell<T>(v: T) -> LCell<T> {
	Rc::new(RefCell::new(v))
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
	Nil,
	Cons((LCell<Value>, LCell<Value>)),
	False,
	True,
	Int(i32),
	Ident(String),
	Fn(Func),
}

impl Value {
	fn head(&self) -> LCell<Value> {
		match *self {
			Value::Cons((ref c,_)) => {
				c.clone()
			},
			ref v => panic!("head called on something not a list! {:?}", v),
		}
	}
	
	fn tail(&self) -> LCell<Value> {
		match *self {
			Value::Cons((_, ref c)) => {
				c.clone()
			},
			ref v => panic!("tail called on something not a list! {:?}", v),
		}
	}

	fn iter(&self) -> ListIterator {
		match *self {
			Value::Cons(_) | Value::Nil => {
				ListIterator{ pos: lcell(self.clone()) }
			},
			ref v => panic!("trying to get an iterator for something not a list! {:?}", v),
		}
	}
}

type HostFunc = fn(LCell<Bindings>) -> LCell<Value>;

#[derive(Clone)]
enum Func {
	NFunc(FunctionDef),
	HFunc(HostFunc),
}

impl Func {
	fn eval(&self, env: LCell<Bindings>) -> LCell<Value> {
		use Func::*;
		match self {
			&NFunc(ref d) => d.eval(env),
			&HFunc(fun) => fun(env),
		}
	}
}

impl std::fmt::Debug for Func {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
		match self {
			&Func::HFunc(_) => write!(f, "<Host Function>"),
			&Func::NFunc(ref def) => write!(f, "{:?}", def),
		}
	}
}

impl PartialEq for Func {
	fn eq(&self, other: &Func) -> bool {
		match self {
			&Func::NFunc(ref def) => match other {
				&Func::NFunc(ref odef) => def == odef,
				_ => false,
			},
			_ => false,
		}
	}
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self { 
			&Func::NFunc(ref fun) => write!(f, "{}", fun),
			&Func::HFunc(_) => write!(f, "<Host Function>"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
struct FunctionDef {
	args: Vec<String>,
	listing: LCell<Value>,
}

impl FunctionDef {
	fn eval(&self, env: LCell<Bindings>) -> LCell<Value> {
		lcell(Value::Nil)
	}
}

impl fmt::Display for FunctionDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(fn (")?;
		let mut first = true;
		for arg in self.args.iter() {
			if first {
				first = false;
			} else {
				write!(f, " ")?;
			}
			write!(f, "{}", arg)?;
		}
		write!(f, ")")?;
		write!(f, "{}", self.listing.borrow())?;
		write!(f, ")")
	}
}

struct ListIterator {
	pos: LCell<Value>,
}

impl Iterator for ListIterator {
	type Item = LCell<Value>;

	fn next(&mut self) -> Option<Self::Item> {
		let retv;
		let new_pos = match &*self.pos.borrow() {
			&Value::Nil => {
				retv = None;
				lcell(Value::Nil)
			},
			&Value::Cons((ref h, ref t)) => {
				retv = Some(h.clone());
				t.clone()
			},
			v => panic!("ListIterator got something strange {:?}", v)
		};
		self.pos = new_pos;
		retv
	}
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Value::Nil => write!(f, "()"),
			Value::False => write!(f, "false"),
			Value::True => write!(f, "true"),
			Value::Int(ref i) => write!(f, "{}", i),
			Value::Ident(ref s) => write!(f, "{}", s),
			Value::Cons(ref inner) => {
				write!(f, "(")?;
				print_list_inner(&*inner, f)?;
				write!(f, ")")
			},
			Value::Fn(ref fun) => write!(f, "{}", fun),
		}
    }
}

fn print_list_inner(inner: &(LCell<Value>, LCell<Value>), f: &mut fmt::Formatter) -> fmt::Result {
	let (ref h, ref t) = *inner;
	write!(f, "{}", h.borrow())?;
	match *t.borrow() {
		Value::Nil => {},
		Value::Cons(ref i) => {
			write!(f, " ")?;
			print_list_inner(i, f)?
		},
		_ => panic!("tis not a list!"),
	}
	write!(f, "")
}


struct ListBuilder {
	head: Option<Value>,
	cur: Value,
}

impl ListBuilder {
	fn new() -> ListBuilder {
		ListBuilder {
			head: None,
			cur: Value::Nil,
		}
	}

	fn build(self) -> Value {
		match self.head {
			None => Value::Nil,
			Some(ret) => ret,
		}
	}

	fn push(&mut self, v: LCell<Value>) {
		let next = (v, lcell(Value::Nil));
		if self.head.is_none() {
			self.head = Some(Value::Cons(next.clone()));
			self.cur = Value::Cons(next);
		} else {
			let new;
			if let Value::Cons(ref c) = self.cur {
				*c.1.borrow_mut() = Value::Cons(next.clone());
				new = Value::Cons(next);
			} else {
				unreachable!();
			}
			self.cur = new;
		}
	}
}

fn truthy(v: Value) -> bool {
	match v {
		Value::Nil | Value::False => false,
		_ => true,
	}
}

fn parse_program<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
	let mut builder = ListBuilder::new();
	for program in pairs {
		for list in program.into_inner() {
			match list.as_rule() {
				Rule::list => {
					builder.push(lcell(parse_list_inner(list.into_inner())));
				}
				_ => panic!("program contains non-list value at top level")
			}
		}
	};
	builder.build()
}

fn parse_list_inner<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
	let mut builder = ListBuilder::new();
	for pair in pairs {
		match pair.as_rule() {
			Rule::term => {
				builder.push(lcell(parse(pair.into_inner())));
			},
			v => panic!("something fishy came along in a list {:?}", v),
		}
	}
	builder.build()
}

fn parse<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
	for pair in pairs {
		match pair.as_rule() {
			Rule::term => {
				return parse(pair.into_inner())
			}
			Rule::list => {
				return parse_list_inner(pair.into_inner())
			},
			Rule::integer => return Value::Int(pair.into_span().as_str().parse().unwrap()),
			Rule::ident => return Value::Ident(String::from(pair.into_span().as_str())),
			v => panic!("wtf: {:?}", v),
		}
	}
	unreachable!()
}

fn read_program(inp: &str) -> Value {
	let pairs = L3Parser::parse_str(Rule::program, inp).unwrap_or_else(|e| panic!("{}", e));
	parse_program(pairs)
}

fn read_list(inp: &str) -> Value {
	let pairs = L3Parser::parse_str(Rule::list, inp).unwrap_or_else(|e| panic!("{}", e));
	parse(pairs)
}

fn cons(head: Value, tail: Value) -> Value {
	Value::Cons((
		Rc::new(RefCell::new(head)),
		Rc::new(RefCell::new(tail))
	))
}

fn int(i: i32) -> Value {
	Value::Int(i)
}

fn ident<T>(i: T) -> Value 
	where T: ToString{
	Value::Ident(i.to_string())
}

fn nil() -> Value {
	Value::Nil
}

use std::collections::BTreeMap;
struct Bindings {
	bindings: BTreeMap<String, LCell<Value>>,
	parent: Option<LCell<Bindings>>,
}

fn make_root_bindings(funs: Vec<(&str, HostFunc)>) -> Bindings {
	let mut bindings = BTreeMap::new();
	for (name, hf) in funs {
		bindings.insert(name.to_string(), lcell(Value::Fn(Func::HFunc(hf))));
	}
	Bindings {
		bindings: bindings,
		parent: None,
	}
}

impl Bindings {
	fn get_binding(&self, id: &Value) -> LCell<Value> {
		if let &Value::Ident(ref i) = id {
			match self.bindings.get(i) {
				None => match self.parent {
					None => lcell(Value::Nil),
					Some(ref parent) => parent.borrow().get_binding(id),
				},
				Some(ref b) => (*b).clone(),
			}
		} else {
			panic!("get_binding called with not an ident")
		}
	}

	fn set_binding(&mut self, id: &Value, v: LCell<Value>) {
		if let &Value::Ident(ref i) = id {
			let mut bind_map = &mut self.bindings;
			bind_map.insert(i.clone(), v);
		} else {
			panic!("set_binding called with not an ident")
		}
	}
}

fn fn_print(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let params = envref.get_binding(&ident("_params"));
	for p in params.borrow().iter() {
		println!("{}", p.borrow());
	}
	lcell(Value::True)
}

fn make_params(params: LCell<Value>, parent: Option<LCell<Bindings>>) -> LCell<Bindings> {
	let mut parmap = BTreeMap::new();
	parmap.insert("_params".to_string(), params);
	lcell(Bindings {
		bindings: parmap,
		parent: parent,
	})
}

fn eval(form: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	use Value::*;
	if let Cons((ref h, ref t)) = *form.borrow() {
		if let Ident(ref id) = *h.borrow() {
			match &**id {
				"quote" => t.clone(),
				"fn" => unimplemented!(),
				_ => {
					let envref = env.borrow();
					let fun_cell = envref.get_binding(&*h.borrow());
					let fnref = fun_cell.borrow();
					if let Value::Fn(ref fun) = *fnref {
						let mut params = ListBuilder::new();
						for p in t.borrow().iter() {
							let evaluated_p = eval(p, env.clone());
							params.push(evaluated_p);
						}
						let fnenv = make_params(lcell(params.build()), Some(env.clone()));
						fun.eval(fnenv)
					} else {
						panic!("function {} not known.", id)
					}
				},
			}
		} else {
			panic!("eval: not a function ident: {:?}", h)
		}
	} else {
		form.clone()
	}
}

fn main() {
	let root_bindings = lcell(make_root_bindings(vec![
		("print", fn_print),
	]));
	
	let program = "(print (quote 1 2 3)) (print A B)";

	for term in read_program(program).iter() {
		eval(term, root_bindings.clone());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn read_print_eq() {
		let exp = "(0 (1 1 ()) (((A) (B C D))))";
		let list = read_list(exp);
		let mut outp = String::new();
		std::fmt::write(&mut outp, format_args!("{}", list));
		assert_eq!(outp, exp.to_string());
	}

	#[test]
	fn basic_parse() {
		assert_eq!(
			read_program("(1) (2 3 (4 5) ((6)))"),
			cons(
				cons(int(1), nil()),
				cons(
					cons(
						int(2), 
						cons(int(3), 
							cons(
								cons(
									int(4), 
									cons(int(5), nil())
								), 
								cons(
									cons(
										cons(int(6),nil()), 
										nil()
									), 
									nil()
								), 
							),
						),
					),
					nil()
				)
			)
		);
	}

	#[test]
	fn head_read() {
		let l = read_list("(1 2 3)");
		let head = l.head();
		assert_eq!(Value::Int(1), *head.borrow());
	}
	
	#[test]
	fn head_write() {
		let l = read_list("(1 2 3)");
		let head = l.head();
		*head.borrow_mut() = Value::Int(4);
		let l2 = read_list("(4 2 3)");
		assert_eq!(l, l2);
	}
	
	#[test]
	fn tail_read() {
		let l = read_list("(1 2 3)");
		let tail = l.tail();
		let l2 = read_list("(2 3)");
		assert_eq!(*tail.borrow(), l2);
	}
	
	#[test]
	fn tail_write() {
		let l = read_list("(1 2 3)");
		let tail = l.tail();
		let new_tail = read_list("(4 5)");
		*tail.borrow_mut() = new_tail;
		let ans = read_list("(1 4 5)");
		assert_eq!(l, ans);
	}

	#[test]
	fn iter_write() {
		let l = read_list("(1 2 3)");
		for b in l.iter() {
			println!("{:?}", b);
			*b.borrow_mut() = int(6);
		}
		let ans = read_list("(6 6 6)");
		assert_eq!(l, ans);
	}

	#[test]
	fn iter_nondestructive() {
		let l = read_list("(1 2 3)");
		for b in l.iter() {
			println!("{:?}", b);
		}
		let ans = read_list("(1 2 3)");
		assert_eq!(l, ans);
	}
}
