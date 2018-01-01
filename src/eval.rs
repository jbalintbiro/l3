use super::*;

pub fn eval(form: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	use Value::*;
	match *form.borrow() {
		Cons((ref h, ref t)) => {
			match *h.borrow() {
				Ident(ref id) => match &**id {
					"quote" => t.clone(),
					"fn" => eval_fn(t.clone(), env.clone()),
					"set" => eval_set(t.clone(), env.clone()),
					"set-global" => eval_set_global(t.clone(), env.clone()),
					"if" => eval_if(t.clone(), env.clone()),
					"for" => eval_for(t.clone(), env.clone()),
					"while" => eval_while(t.clone(), env.clone()),
					"and" => eval_and(t.clone(), env.clone()),
					"or" => eval_or(t.clone(), env.clone()),
					_ => {
						eval_fncall(h.clone(), t.clone(), env.clone())
					},
				},
				_ => panic!("eval: not a function ident: {}", &*h.borrow())
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

fn eval_fncall(fun_name: LCell<Value>, arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let fnref;
	{
		let envref = env.borrow();
		let fun_cell = envref.get_binding(&*fun_name.borrow());
		fnref = fun_cell.borrow().clone();
	}
	if let Value::Fn(ref fun) = fnref {
		let mut params = ListBuilder::new();
		for arg in arguments.borrow().iter() {
			params.push(eval(arg, env.clone()));
		}
		fun.eval(lcell(params.build()), env.clone())
	} else {
		panic!("function `{}` not known.", &*fun_name.borrow())
	}
}

