use super::*;

pub fn default_root() -> LCell<Bindings> {
	lcell(make_root_bindings(vec![
		("print", fn_print),
		("list", fn_list),
		("exit", fn_exit),
		("+", fn_add),
		("*", fn_mul),
		("-", fn_sub),
		("=", fn_eq),
		("<", fn_lt),
		("<=", fn_le),
		("!=", fn_ne),
		(">", fn_gt),
		(">=", fn_ge),
	]))
}

fn int_iter<I: Iterator<Item=LCell<Value>>>(it: I) -> impl Iterator<Item=i32> {
	it.map(|v| {
		match &*v.borrow() {
			&Value::Int(i) => i,
			v => panic!("parameters contain something not an integer {}", v),
		}
	})
}

fn fn_print(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	for p in params.borrow().iter() {
		println!("{}", p.borrow());
	}
	lcell(Value::True)
}

fn fn_list(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	lcell(params.borrow().iter().collect::<Value>())
}

fn fn_exit(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let code = match params.borrow().iter().next() {
		None => 0,
		Some(v) => match *v.borrow() {
			Value::Int(i) => i,
			ref v => panic!("exit called with something other than an integer {}", v)
		}
	};
	std::process::exit(code);
}

fn fn_add(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	lcell(int(int_iter(params.borrow().iter()).sum()))
}

fn fn_mul(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	lcell(int(int_iter(params.borrow().iter()).product()))
}

fn fn_sub(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let mut it = int_iter(params.borrow().iter());
	if let Some(mut acc) = it.next() {
		lcell(int(match it.next() {
			None => -1 * acc,
			Some(n) => {
				acc /= n;
				for n in it {
					acc /= n
				}
				acc
			}
		}))
	} else {
		panic!("sub called without a parameter")
	}
}

macro_rules! make_comparison {
	($func:ident, $invert:tt) => (
		fn $func(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
			let mut it = params.borrow().iter();
			let v0 = it.next().expect("comparison called without parameters");
			for v in it {
				if v $invert v0 { return lcell(Value::False); }
			}
			lcell(Value::True)
		}
	)
}

make_comparison!(fn_eq, !=);
make_comparison!(fn_lt, >=);
make_comparison!(fn_le, >);
make_comparison!(fn_ne, ==);
make_comparison!(fn_gt, <=);
make_comparison!(fn_ge, <);
