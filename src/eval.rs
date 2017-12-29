use super::*;

pub fn eval(form: LCell<Value>, env: LCell<Bindings>) -> LCell<Value> {
	use Value::*;
	if let Cons((ref h, ref t)) = *form.borrow() {
		if let Ident(ref id) = *h.borrow() {
			match &**id {
				"quote" => t.clone(),
				"fn" => unimplemented!(),
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
						let fnenv = make_params(lcell(params.build()), Some(env.clone()));
						fun.eval(fnenv)
					} else {
						panic!("function `{}` not known.", id)
					}
				},
			}
		} else {
			panic!("eval: not a function ident: {:?}", h)
		}
	} else {
		form.clone()
	}
}
