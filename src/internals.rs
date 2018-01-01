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

pub fn nil() -> LCell<Value> {
	lcell(Value::Nil)
}

pub fn boolean(b: bool) -> LCell<Value> {
	lcell(if b {
		Value::True
	} else {
		Value::False
	})
}

