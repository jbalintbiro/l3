use std::collections::BTreeMap;
use super::*;

#[derive(Clone, Debug, PartialEq, Trace, Finalize)]
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

	pub fn set_root_binding(&mut self, id: &Value, v: LCell<Value>) {
		if let Some(ref mut parent) = self.parent {
			parent.borrow_mut().set_root_binding(id, v);
		} else {
			if let &Value::Ident(ref i) = id {
				let mut bind_map = &mut self.bindings;
				bind_map.insert(i.clone(), v);
			} else {
				panic!("set_binding called with not an ident")
			}
		}
	}
}

pub fn make_root_bindings(funs: Vec<(&str, HostFunc)>, vals: Vec<(&str, LCell<Value>)>) -> Bindings {
	let mut bindings = BTreeMap::new();
	for (name, val) in vals {
		bindings.insert(name.to_string(), val);
	}
	for (name, hf) in funs {
		bindings.insert(name.to_string(), lcell(Value::Fn(Func::HFunc(hf))));
	}
	Bindings {
		bindings: bindings,
		parent: None,
	}
}

pub fn make_empty_bindings(parent: LCell<Bindings>) -> Bindings {
	Bindings {
		bindings: BTreeMap::new(),
		parent: Some(parent),
	}
}
