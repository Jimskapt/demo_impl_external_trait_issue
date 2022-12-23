pub trait MyTrait {
	fn my_function(&mut self, _arguments: crate::Arguments) -> crate::Result;
}

#[test]
fn test_trait() {
	let mut an_implement = demo_implement::MyImplement {};

	assert_eq!(
		MyTrait::my_function(&mut an_implement, Arguments {}),
		Result {}
	);
}

pub struct Arguments {}

#[derive(PartialEq, Debug)]
pub struct Result {}
