use std::collections::BTreeMap;
use super::*;

pub struct Bindings {
	bindings: BTreeMap<String, LCell<Value>>,
	parent: Option<LCell<Bindings>>,
}

impl Bindings {
	pub fn get_binding(&self, id: &Value) -> LCell<Value> {
		if let &Value::Ident(ref i) = id {
			match self.bindings.get(i) {
				None => match self.parent {
					None => lcell(Value::Nil),
					Some(ref parent) => parent.borrow().get_binding(id),
				},
				Some(ref b) => (*b).clone(),
			}
		} else {
			panic!("get_binding called with not an ident")
		}
	}

	pub fn set_binding(&mut self, id: &Value, v: LCell<Value>) {
		if let &Value::Ident(ref i) = id {
			let mut bind_map = &mut self.bindings;
			bind_map.insert(i.clone(), v);
		} else {
			panic!("set_binding called with not an ident")
		}
	}
}

pub fn make_root_bindings(funs: Vec<(&str, HostFunc)>) -> Bindings {
	let mut bindings = BTreeMap::new();
	for (name, hf) in funs {
		bindings.insert(name.to_string(), lcell(Value::Fn(Func::HFunc(hf))));
	}
	Bindings {
		bindings: bindings,
		parent: None,
	}
}

pub fn make_params(params: LCell<Value>, parent: Option<LCell<Bindings>>) -> LCell<Bindings> {
	let mut parmap = BTreeMap::new();
	parmap.insert("_params".to_string(), params);
	lcell(Bindings {
		bindings: parmap,
		parent: parent,
	})
}