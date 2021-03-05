#![allow(non_snake_case)]

/*
#![no_std]

#[panic_handler]
fn panicHandler(_info: &core::panic::PanicInfo) -> !
{
    loop {}
}*/

struct TestStruct
{
	a: i32,
	b: u16
}

#[export_name = "testFn"]
#[no_mangle]
pub fn testFn() -> i32
{
	let mut str: TestStruct = TestStruct {a: 64, b: 10};

	str.a = 18;
	str.b = 3;

	return str.a + str.b as i32;
}