use super::*;

pub fn lcell<T>(v: T) -> LCell<T>
	where T: gc::Trace + gc::Finalize {
    Gc::new(GcCell::new(v))
}

pub fn cons(head: LCell<Value>, tail: LCell<Value>) -> LCell<Value> {
	lcell(Value::Cons((head, tail)))
}

pub fn int(i: i32) -> LCell<Value> {
	lcell(Value::Int(i))
}

pub fn ident<T>(i: T) -> LCell<Value>
	where T: ToString{
	lcell(Value::Ident(i.to_string()))
}

thread_local! {
	static NIL: LCell<Value> = lcell(Value::Nil);
	static FALSE: LCell<Value> = lcell(Value::False);
	static TRUE: LCell<Value> = lcell(Value::True);
	static EOF: LCell<Value> = lcell(Value::EOF);
}

pub fn nil() -> LCell<Value> {
	NIL.with(|nil| nil.clone())
}

pub fn boolean(b: bool) -> LCell<Value> {
	if b {
		TRUE.with(|t| t.clone())
	} else {
		FALSE.with(|f| f.clone())
	}
}

pub fn eof() -> LCell<Value> {
	EOF.with(|eof| eof.clone())
}

