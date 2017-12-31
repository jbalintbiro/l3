use super::*;

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

pub fn fn_exit(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
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

pub fn default_root() -> LCell<Bindings> {
	lcell(make_root_bindings(vec![
		("print", fn_print),
		("list", fn_list),
		("exit", fn_exit),
		("+", fn_add),
		("*", fn_mul),
		("-", fn_sub),
	]))
}
