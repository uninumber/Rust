fn main() {

    let s ="hello";
    //let b = &s[3];

    println!("The third element of s is: {:?}", s.chars().nth(3));
    //Accessing the element of str is just forbidden, even if you will try to get access from
    //&str;
    
    //If you know how much data the String will hold use: with_capacity()
}

