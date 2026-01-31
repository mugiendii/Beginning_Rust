fn main() {
    //ownership examples
    //memory management, variable scope, borrowing, references, mutable references

    //ownership rules
    //1. each value in rust has a variable that is its owner
    //2. there can only be one owner at a time
    //3. when the owner goes out of scope, the value will be dropped

    //rule1
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //pass by reference using &
    println!("The length of '{}' is {}.", s1, len);


    //rule2
    let s2 = String::from("world");
    let s3 = s2; //s2 is moved to s3, s2
    println!("s3: {}", s3);
    // println!("s2: {}", s2); //error: borrow of moved value:


    //rule3
    
    let s4 = String::from("Scoped String");
    println!("s4 inside scope: {}", s4);
    //s4 goes out of scope here and is dropped
    //print_string(s4);
}
fn print_string(s: String) {
    println!("{}", &s4);
}

fn calculate_length(s: &String) -> usize {
    s.len()

}