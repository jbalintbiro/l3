use pest::inputs::Input;
use super::*;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("l3.pest"); // relative to this file

#[derive(Parser)]
#[grammar = "l3.pest"]
pub struct L3Parser;

pub fn parse_program<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
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

pub fn parse_list_inner<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
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

pub fn parse<I: Input>(pairs: pest::iterators::Pairs<Rule, I>) -> Value {
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
