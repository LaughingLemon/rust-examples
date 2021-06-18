
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        //unsafe allows us to set the mutable references above
        *r2 = 12;

        println!("r1: {}, r2: {}", *r1, *r2);
    }
}

