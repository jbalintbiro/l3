	use super::*;
	
	#[test]
	fn basic_eval() {
		assert_eq!(
			eval(lcell(read_list("(+ (* 2 3) 1)")), default_root()),
			int(7)
		);
	}

	#[test]
	fn read_print_eq() {
		let exp = "(0 (1 1 ()) (((A) (B C D))))";
		let list = read_list(exp);
		let mut outp = String::new();
		std::fmt::write(&mut outp, format_args!("{}", list)).unwrap();
		assert_eq!(outp, exp.to_string());
	}

	#[test]
	fn basic_parse() {
		let program = read_program("(1) (2 3 (4 5) ((6)))");
		let evaluated = cons(
			cons(int(1), nil()),
			cons(
				cons(
					int(2), 
					cons(int(3), 
						cons(
							cons(
								int(4), 
								cons(int(5), nil())
							), 
							cons(
								cons(
									cons(int(6),nil()), 
									nil()
								), 
								nil()
							), 
						),
					),
				),
				nil()
			)
		);
		assert_eq!(program, *evaluated.borrow());
	}

	#[test]
	fn head_read() {
		let l = read_list("(1 2 3)");
		let head = l.head();
		assert_eq!(Value::Int(1), *head.borrow());
	}
	
	#[test]
	fn head_write() {
		let l = read_list("(1 2 3)");
		let head = l.head();
		*head.borrow_mut() = Value::Int(4);
		let l2 = read_list("(4 2 3)");
		assert_eq!(l, l2);
	}
	
	#[test]
	fn tail_read() {
		let l = read_list("(1 2 3)");
		let tail = l.tail();
		let l2 = read_list("(2 3)");
		assert_eq!(*tail.borrow(), l2);
	}
	
	#[test]
	fn tail_write() {
		let l = read_list("(1 2 3)");
		let tail = l.tail();
		let new_tail = read_list("(4 5)");
		*tail.borrow_mut() = new_tail;
		let ans = read_list("(1 4 5)");
		assert_eq!(l, ans);
	}

	#[test]
	fn iter_write() {
		let l = read_list("(1 2 3)");
		for b in l.iter() {
			println!("{:?}", b);
			*b.borrow_mut() = Value::Int(6);
		}
		let ans = read_list("(6 6 6)");
		assert_eq!(l, ans);
	}

	#[test]
	fn iter_nondestructive() {
		let l = read_list("(1 2 3)");
		for b in l.iter() {
			println!("{:?}", b);
		}
		let ans = read_list("(1 2 3)");
		assert_eq!(l, ans);
	}
