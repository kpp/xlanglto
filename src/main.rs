#[link(name="mul", kind="static")]
extern {
    fn mul(a: u32, b: u32) -> u32;
}

#[inline(never)]
#[no_mangle]
fn multiply(a: u32, b: u32) -> u32 {
    unsafe { mul(a, b) }
}

fn main() {
    println!("42*42 = {}!", multiply(42,42));
    println!("87*31 = {}!", multiply(87,31));
}
