use super::*;

pub fn root_macros() -> Vec<(&'static str, HostFunc)> {
	vec![
		("quote", eval_quote),
		("fn", eval_fn),
		("set", eval_set),
		("set-global", eval_set_global),
		("if", eval_if),
		("for", eval_for),
		("while", eval_while),
		("and", eval_and),
		("or", eval_or),
		("loop", eval_loop),
	]
}

pub fn eval(form: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	use Value::*;
	match *form.borrow() {
		Cons((ref h, ref t)) => {
			let evaluated = eval(h.clone(), env.clone());
			let evref = evaluated.borrow();
			match *evref {
				Value::Fn(ref fun) => {
					fun.eval(eval_args(t.clone(), env.clone()), env.clone())
				},
				Value::Macro(ref mac) => {
					mac.eval(t.clone(), env.clone())
				},
				ref v => {
					panic!("{} found in function position", *v)
				}
			}
		},
		Value::Ident(ref i) => {
			let envref = env.borrow();
			let cell = envref.get_binding(&Value::Ident(i.clone()));
			cell
		}
		_ => form.clone()
	}
}

fn eval_quote(params: LCell<Value>, _env: LCell<Bindings>) -> LCell<Value> {
	params.clone()
}

fn eval_args(params: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	lcell(params.borrow().iter().map(|expr| eval(expr, env.clone())).collect())
}

fn eval_and(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut retval = nil();
	for expr in arguments.borrow().iter() {
		retval = eval(expr, env.clone());
		if !retval.borrow().truthy() {
			return nil();
		}
	}
	retval
}

fn eval_or(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	for expr in arguments.borrow().iter() {
		let value = eval(expr, env.clone());
		if value.borrow().truthy() {
			return value.clone();
		}
	}
	nil()
}

fn eval_loop(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let it = arguments.borrow().iter();
	loop {
		let expr_it = it.clone();
		for _ in expr_it.map(|expr| eval(expr, env.clone())) {}
	}
}

fn eval_while(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	let predicate = it.next().expect("while needs a predicate");
	let mut retval = nil();
	loop {
		let evaluated_p = eval(predicate.clone(), env.clone());
		if !evaluated_p.borrow().truthy() {
			break;
		}
		let expr_it = it.clone();
		for ret in expr_it.map(|expr| eval(expr, env.clone())) {
			retval = ret;
		}
	};
	retval
}

fn eval_for(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	if let Some(ref name) = it.next() {
		if let Value::Ident(ref id) = *name.borrow() {
			let evaluated_list = eval(it.next().expect(""), env.clone());
			let list_it = evaluated_list.borrow().iter();
			let mut retval = ListBuilder::new();
			for elem in list_it {
				env.borrow_mut().set_binding(&Value::Ident(id.clone()), elem);
				let expr_it = it.clone();
				let mut retval_candidate = nil();
				for rc in expr_it.map(|expr| eval(expr, env.clone())) {
					retval_candidate = rc;
				}
				retval.push(retval_candidate);
			}
			lcell(retval.build())
		} else {
			panic!("Identifier expected in for expr binding")
		}
	} else {
		panic!("arguments expected for for expr")
	}
}


fn eval_if(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	let predicate = it.next().expect("if needs a predicate");
	let true_branch = it.next().expect("if needs a true branch");
	let maybe_false_branch = it.next();

	let p_eval = eval(predicate, env.clone());
	if p_eval.borrow().truthy() {
		eval(true_branch, env)
	} else {
		if let Some(false_branch) = maybe_false_branch {
			eval(false_branch, env)
		} else {
			nil()
		}
	}
}

fn eval_set_global(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	if let Some(first) = it.next() {
		if let Value::Ident(_) = *first.borrow() {
			let evaluated = eval(it.next().unwrap(), env.clone());
			(*env.borrow_mut()).set_root_binding(&first.borrow(), evaluated.clone());
			evaluated
		} else {
			panic!("set-global got something else than an identifier")
		}
	} else {
		panic!("set-global called without parameters")
	}
}

fn eval_set(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	if let Some(first) = it.next() {
		if let Value::Ident(_) = *first.borrow() {
			let evaluated = eval(it.next().unwrap(), env.clone());
			(*env.borrow_mut()).set_binding(&first.borrow(), evaluated.clone());
			evaluated
		} else {
			panic!("set got something else than an identifier")
		}
	} else {
		panic!("set called without parameters")
	}
}

fn eval_fn(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	let first = it.next().expect("fn called without arguments");
	let bind = if let Value::Ident(_) = *first.borrow() {
		Some(first.borrow())
	} else { None };

	let arglist = match bind {
		Some(_) => it.next().expect("fn called without argument list"),
		None => first.clone(),
	};

	let argvec: Vec<String> = arglist.borrow().iter()
								.map(|ar| {
									match *ar.borrow() {
										Value::Ident(ref i) => i.clone(),
										ref v => panic!("fn argument list containing something not an ident {}", v),
									}
								})
								.collect();

	let listing = lcell(it.collect());

	let fun = lcell(Value::Fn(Func::NFunc(FunctionDef{
		args: argvec,
		listing: listing,
		env: lcell(make_empty_bindings(env.clone())),
	})));

	if let Some(binding) = bind {
		(*env.borrow_mut()).set_binding(&binding, fun);
		nil()
	} else {
		fun
	}
}

