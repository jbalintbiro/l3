use super::*;

pub fn lcell<T>(v: T) -> LCell<T> 
	where T: gc::Trace + gc::Finalize {
    Gc::new(GcCell::new(v))
}

pub fn cons(head: Value, tail: Value) -> Value {
	Value::Cons((lcell(head), lcell(tail)))
}

pub fn int(i: i32) -> Value {
	Value::Int(i)
}

pub fn ident<T>(i: T) -> Value 
	where T: ToString{
	Value::Ident(i.to_string())
}

pub fn nil() -> Value {
	Value::Nil
}

