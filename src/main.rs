extern crate libc;

#[link(name="hello")]
extern {
	fn hello(args: &str) -> i32;
}

fn main(){
	let input = "hello, cffi!";
	unsafe{
		hello(&input);
		}
}