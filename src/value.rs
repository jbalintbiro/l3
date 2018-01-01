use super::*;

pub type LCell<T> = Gc<GcCell<T>>;

#[derive(Debug, PartialEq, Clone, PartialOrd, Trace, Finalize)]
pub enum Value {
	Nil,
	Cons((LCell<Value>, LCell<Value>)),
	False,
	True,
	Int(i32),
	Ident(String),
	Fn(Func),
	Macro(Func),
	EOF,
}

impl Value {
	pub fn head(&self) -> LCell<Value> {
		match *self {
			Value::Cons((ref c,_)) => {
				c.clone()
			},
			ref v => panic!("head called on something not a list! {:?}", v),
		}
	}

	pub fn tail(&self) -> LCell<Value> {
		match *self {
			Value::Cons((_, ref c)) => {
				c.clone()
			},
			ref v => panic!("tail called on something not a list! {:?}", v),
		}
	}

	pub fn iter(&self) -> ListIterator {
		match *self {
			Value::Cons(_) | Value::Nil => {
				ListIterator{ pos: lcell(self.clone()) }
			},
			ref v => panic!("trying to get an iterator for something not a list! {:?}", v),
		}
	}

	pub fn truthy(&self) -> bool {
		match self {
			&Value::Nil | &Value::False => false,
			_ => true,
		}
	}
}

#[derive(Clone)]
pub struct ListIterator {
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
			Value::Fn(ref fun) => write!(f, "(fn {})", fun),
			Value::Macro(ref fun) => write!(f, "(macro {})", fun),
			Value::EOF => write!(f, "EOF"),
		}
    }
}

fn print_list_inner(inner: &(LCell<Value>, LCell<Value>), f: &mut fmt::Formatter) -> fmt::Result {
	let (ref h, ref t) = *inner;
	write!(f, "{}", &*h.borrow())?;
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

impl FromIterator<LCell<Value>> for Value {
	fn from_iter<I: IntoIterator<Item=LCell<Value>>>(iter: I) -> Self {
        let mut builder = ListBuilder::new();
        for v in iter {
            builder.push(v);
        }
		builder.build()
    }
}

pub struct ListBuilder {
	head: Option<Value>,
	cur: Value,
}

impl ListBuilder {
	pub fn new() -> ListBuilder {
		ListBuilder {
			head: None,
			cur: Value::Nil,
		}
	}

	pub fn build(self) -> Value {
		match self.head {
			None => Value::Nil,
			Some(ret) => ret,
		}
	}

	pub fn push(&mut self, v: LCell<Value>) {
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
