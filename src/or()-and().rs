fn main() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;
  
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    println!("{:?} {:?}", s1.or(s2), s1);
    println!("{:?} {:?}", s1.or(s2), s1); // Some1 or Some2 = Some1
    println!("{:?} {:?}", s1.or(n), s1);  // Some or None = Some
    println!("{:?} {:?}", n.or(s1), s1);  // None or Some = Some
    println!("{:?} {:?}", n.or(n), n);    // None1 or None2 = None2
  
    println!("{:?} {:?}", o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    println!("{:?} {:?}", o1.or(e1), o1); // Ok or Err = Ok
    println!("{:?} {:?}", e1.or(o1), o1); // Err or Ok = Ok
    println!("{:?} {:?}", e1.or(e2), e2); // Err1 or Err2 = Err2
  
    println!("{:?} {:?}", s1.and(s2), s2); // Some1 and Some2 = Some2
    println!("{:?} {:?}", s1.and(n), n);   // Some and None = None
    println!("{:?} {:?}", n.and(s1), n);   // None and Some = None
    println!("{:?} {:?}", n.and(n), n);    // None1 and None2 = None1
  
    println!("{:?} {:?}", o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    println!("{:?} {:?}", o1.and(e1), e1); // Ok and Err = Err
    println!("{:?} {:?}", e1.and(o1), e1); // Err and Ok = Err
    println!("{:?} {:?}", e1.and(e2), e1); // Err1 and Err2 = Err
  }