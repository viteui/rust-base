fn main() {
    let o1: Result<&str, &str> = Ok("1");
    let o2: Result<i32, isize> = Ok(1); 

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize
    println!("{:?} {:?}", o1.map(fn_character_count), o2); 
    println!("{:?} {:?}", o1.map_err(fn_character_count), o2); 
    println!("{:?} {:?}", e1.map_err(fn_character_count), e2);
    println!("{:?} {:?}", e1.map(fn_character_count), e2);
}
