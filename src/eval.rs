use super::*;

fn eval_fn(arguments: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	let mut it = arguments.borrow().iter();
	let first = it.next().expect("fn called without arguments");
	let bind = if let Value::Ident(ref id) = *first.borrow() {
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
										ref v => panic!("fn argument list containing something not an ident {v}"),
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
		lcell(nil())
	} else {
		fun
	}
}

pub fn eval(form: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	use Value::*;
	match *form.borrow() {
		Cons((ref h, ref t)) => {
			if let Ident(ref id) = *h.borrow() {
				match &**id {
					"quote" => t.clone(),
					"fn" => {
						eval_fn(t.clone(), env.clone())
					},
					_ => {
						let envref = env.borrow();
						let fun_cell = envref.get_binding(&*h.borrow());
						let fnref = fun_cell.borrow();
						if let Value::Fn(ref fun) = *fnref {
							let mut params = ListBuilder::new();
							for p in t.borrow().iter() {
								let evaluated_p = eval(p, env.clone());
								params.push(evaluated_p);
							}
							fun.eval(lcell(params.build()), env.clone())
						} else {
							panic!("function `{}` not known.", id)
						}
					},
				}
			} else {
				panic!("eval: not a function ident: {:?}", h)
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
