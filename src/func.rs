use super::*;

pub type HostFunc = fn(LCell<Bindings>) -> LCell<Value>;

#[derive(Clone)]
pub enum Func {
	NFunc(FunctionDef),
	HFunc(HostFunc),
}

impl Func {
	pub fn eval(&self, env: LCell<Bindings>) -> LCell<Value> {
		use self::Func::*;
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
pub struct FunctionDef {
	args: Vec<String>,
	listing: LCell<Value>,
}

impl FunctionDef {
	pub fn eval(&self, env: LCell<Bindings>) -> LCell<Value> {
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
