use super::*;

fn int_iter<I: Iterator<Item=LCell<Value>>>(it: I) -> impl Iterator<Item=i32> {
	it.map(|v| {
		match &*v.borrow() {
			&Value::Int(i) => i,
			v => panic!("parameters contain something not an integer {}", v),
		}
	})
}

fn fn_print(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let params = envref.get_binding(&ident("_params"));
	for p in params.borrow().iter() {
		println!("{}", p.borrow());
	}
	lcell(Value::True)
}

fn fn_list(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let paramsref = envref.get_binding(&ident("_params"));
	let params = paramsref.borrow();
	lcell(params.iter().collect::<Value>())
}

pub fn fn_exit(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let paramsref = envref.get_binding(&ident("_params"));
	let params = paramsref.borrow();
	let code = match params.iter().next() {
		None => 0,
		Some(v) => match *v.borrow() {
			Value::Int(i) => i,
			ref v => panic!("exit called with something other than an integer {}", v)
		}
	};
	std::process::exit(code);
}

fn fn_add(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let paramsref = envref.get_binding(&ident("_params"));
	let params = paramsref.borrow();
	lcell(int(int_iter(params.iter()).sum()))
}

fn fn_mul(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let paramsref = envref.get_binding(&ident("_params"));
	let params = paramsref.borrow();
	lcell(int(int_iter(params.iter()).product()))
}

fn fn_sub(env: LCell<Bindings>) -> LCell<Value> {
	let envref = env.borrow();
	let paramsref = envref.get_binding(&ident("_params"));
	let params = paramsref.borrow();
	let mut it = int_iter(params.iter());
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

//fn fn_div;
//fn fn_mod;

pub fn default_root() -> LCell<Bindings> {
	lcell(make_root_bindings(vec![
		("print", fn_print),
		("list", fn_list),
		("exit", fn_exit),
		("+", fn_add),
		("*", fn_mul),
		("-", fn_sub),
	//	("/", fn_div),
	//	("mod", fn_mod),
	]))
}
