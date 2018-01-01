use super::*;

pub fn default_root() -> LCell<Bindings> {
	lcell(make_root_bindings(vec![
		("print", fn_print),
		("eval", fn_eval),
		("read", fn_read),

		("cons", fn_cons),
		("list", fn_list),
		("head", fn_head),
		("tail", fn_tail),
		("cat", fn_cat),
		("#", fn_idx),

		("+", fn_add),
		("*", fn_mul),
		("-", fn_sub),
		("/", fn_div),

		("=", fn_eq),
		("<", fn_lt),
		("<=", fn_le),
		("!=", fn_ne),
		(">", fn_gt),
		(">=", fn_ge),

		("exit", fn_exit),
	],
	vec![
		("EOF", eof()),
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

fn fn_read(_params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	read_stdin()
}

fn fn_eval(params: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = params.borrow().iter();
	eval(it.next().expect("eval called without parameter"), env)
}

fn fn_cons(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let mut it = params.borrow().iter();
	let h = it.next().expect("cons called with less than 2 arguments").clone();
	let t = it.next().expect("cons called with less than 2 arguments").clone();
	cons(h, t)
}

fn fn_head(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let list = params.borrow().iter().next().expect("head called without a parameter");
	let rf = list.borrow();
	rf.head().clone()
}

fn fn_tail(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let list = params.borrow().iter().next().expect("tail called without a parameter");
	let rf = list.borrow();
	rf.tail().clone()
}

fn fn_idx(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let mut it = params.borrow().iter();
	let first = it.next().expect("idx called without parameters");
	let idx = if let Value::Int(n) = *first.borrow() {
		n
	} else {
		panic!("idx called with non-numeric index")
	};

	if idx < 1 {
		panic!("idx called with wrong index")
	}

	let list = it.next().expect("idx missing list");
	let mut list_it = list.borrow().iter();

	let mut count = idx;
	let mut retval = nil();
	while count > 0 {
		retval = list_it.next().expect("idx called with fewer elements");
		count -= 1;
	}
	retval
}

fn fn_print(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	for p in params.borrow().iter() {
		println!("{}", &*p.borrow());
	}
	boolean(true)
}

fn fn_list(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	lcell(params.borrow().iter().collect::<Value>())
}

fn fn_cat(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let mut builder = ListBuilder::new();
	for p in params.borrow().iter() {
		for e in p.borrow().iter() {
			builder.push(e);
		}
	}
	lcell(builder.build())
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
	int(int_iter(params.borrow().iter()).sum())
}

fn fn_mul(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	int(int_iter(params.borrow().iter()).product())
}

fn fn_sub(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let mut it = int_iter(params.borrow().iter());
	if let Some(mut acc) = it.next() {
		int(match it.next() {
			None => -1 * acc,
			Some(n) => {
				acc -= n;
				for n in it {
					acc -= n
				}
				acc
			}
		})
	} else {
		panic!("sub called without a parameter")
	}
}

fn fn_div(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	let mut it = int_iter(params.borrow().iter());
	if let Some(mut acc) = it.next() {
		int(match it.next() {
			None => panic!("div got less than 2 parameters"),
			Some(n) => {
				acc /= n;
				for n in it {
					acc /= n
				}
				acc
			}
		})
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
				if v $invert v0 { return boolean(false); }
			}
			boolean(true)
		}
	)
}

make_comparison!(fn_eq, !=);
make_comparison!(fn_lt, >=);
make_comparison!(fn_le, >);
make_comparison!(fn_ne, ==);
make_comparison!(fn_gt, <=);
make_comparison!(fn_ge, <);
