use super::*;

pub fn read_program(inp: &str) -> Value {
	let pairs = L3Parser::parse_str(Rule::program, inp).unwrap_or_else(|e| panic!("{}", e));
	parse_program(pairs)
}

pub fn read_program_file(file: &str) -> Value {
	use pest::inputs::FileInput;
	let pairs = L3Parser::parse(
		Rule::program,
		Rc::new(FileInput::new(file).unwrap_or_else(|e| panic!("{}", e))
	)).unwrap_or_else(|e| panic!("{}", e));
	parse_program(pairs)
}

pub fn read_list(inp: &str) -> Value {
	let pairs = L3Parser::parse_str(Rule::list, inp).unwrap_or_else(|e| panic!("{}", e));
	parse(pairs)
}

use std::sync::Mutex;
lazy_static!{
	static ref INBUF: Mutex<String> = Mutex::new(String::new());
}

pub fn read_stdin() -> LCell<Value> {
	use std::io;
	{
		while !L3Parser::parse_str(Rule::list, &INBUF.lock().expect("STDIN BUFFER POISIONED!")).is_ok() {
			let readlen = io::stdin().read_line(&mut INBUF.lock().expect("STDIN BUFFER POISIONED!"))
				.expect("stdin read error");
			if readlen == 0 {
				return eof()
			}
		}
	}
	let good: String = { INBUF.lock().unwrap().clone() };
	match L3Parser::parse_str(Rule::list, &good) {
		Ok(pairs) => {
			if let Some(pair) = pairs.clone().next() {
				let len = pair.into_span().end();
				let remainder = { INBUF.lock().unwrap().split_off(len) };
				{ *INBUF.lock().unwrap() = remainder; }
				lcell(parse(pairs))
			} else {
				panic!("read: something deeply wrong")
			}
		}
		_ => nil(),
	}
}

