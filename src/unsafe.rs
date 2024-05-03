fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    unsafe {
        // let r2 = &mut num as *mut i32;
        println!("{}", *r1);
    }
    num += 5;
    println!("{}", num);
}
