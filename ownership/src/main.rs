fn main() {
    let string_literal = "hello";
    {
        // s_inner is not valid here, it’s not yet declared
        let s_inner = "hello"; // s_inner is valid from this point forward
                               // do stuff with s_inner
        println!("{}, {}", string_literal, s_inner);
    } // this scope is now over, and s_inner is no longer valid
      //copy integer values
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    //
    let mut string_type = String::from("hello");
    string_type.push_str(", world!");
    let s2 = string_type;
    //string_type is no longer valid here, because it was moved to s2, so it csn not be used.
    //println!("{}, world!", string_type);

    //copy data on stack and on heap
    let s3 = s2.clone();
    println!("s1 = {}, s2 = {}", s2, s3);
    //transfering owneship to funtion
    takes_ownership(s3);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
