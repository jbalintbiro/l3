use super::*;

pub fn read_program(inp: &str) -> Value {
	let pairs = L3Parser::parse_str(Rule::program, inp).unwrap_or_else(|e| panic!("{}", e));
	parse_program(pairs)
}

pub fn read_list(inp: &str) -> Value {
	let pairs = L3Parser::parse_str(Rule::list, inp).unwrap_or_else(|e| panic!("{}", e));
	parse(pairs)
}

