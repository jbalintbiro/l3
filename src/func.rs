use super::*;
use std::cmp::Ordering;

pub type HostFunc = fn(LCell<Value>, LCell<Bindings>) -> LCell<Value>;

#[derive(Clone, Finalize, Trace)]
pub enum Func {
	NFunc(FunctionDef),
	HFunc(HostFunc),
}

impl Func {
	pub fn eval(&self, params: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
		use self::Func::*;
		match self {
			&NFunc(ref d) => d.eval(params, env),
			&HFunc(fun) => fun(params, env),
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

impl PartialOrd for Func {
	fn partial_cmp(&self, _other: &Func) -> Option<Ordering> {
		None
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

#[derive(Clone, PartialEq, Trace, Finalize)]
pub struct FunctionDef {
	pub args: Vec<String>,
	pub listing: LCell<Value>,
	pub env: LCell<Bindings>,
}

impl fmt::Debug for FunctionDef {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "FunctionDef {{ args: {:?}, listing: {:?} }}", self.args, self.listing)
	}
}

impl FunctionDef {
	pub fn eval(&self, params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
		let mut func_env = make_empty_bindings(self.env.clone());
		let mut it = params.borrow().iter();
		for arg_name in self.args.iter() {
			func_env.set_binding(&Value::Ident(arg_name.clone()), it.next().expect("not enough params"))
		}
		let func_env_boxed = lcell(func_env);
		let mut retval = nil();
		for expr in self.listing.borrow().iter() {
			retval = ::eval(expr, func_env_boxed.clone());
		}
		retval
	}
}

impl fmt::Display for FunctionDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(")?;
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
		write!(f, "{}", &*self.listing.borrow())
	}
}
