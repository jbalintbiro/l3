use super::*;

pub fn lcell<T>(v: T) -> LCell<T> {
    Rc::new(RefCell::new(v))
}

pub fn truthy(v: Value) -> bool {
	match v {
		Value::Nil | Value::False => false,
		_ => true,
	}
}

pub fn cons(head: Value, tail: Value) -> Value {
	Value::Cons((
		Rc::new(RefCell::new(head)),
		Rc::new(RefCell::new(tail))
	))
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

