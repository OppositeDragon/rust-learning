fn main() {
    let r;
    let x = 5; // ----------+-- 'b
               //           |
    r = &x; // --+-- 'a  |
            //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
    longest("", "");

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

		Rectangle{width:25, height:25}.can_hold( &Rectangle{width:10, height:10});
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    print!("{}", y);
    x
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
