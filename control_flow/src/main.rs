


fn main() {
   

//if expression example
let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
 //if in a let statement example
    let condition = true;
    let number = if condition { 5 } else { 6 }; 
    println!("The value of number is: {}", number);
//loops
//used to retryab operation until a certain condition is met
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 10 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    //while loop example
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }


    //loopig through an array using for loop
    let a = [10, 20, 30, 40, 50];
    let b = ["a", "b", "c", "d", "e"];
    for element in a {
        println!("the value is: {}", element);  
    }
    for letter in b {
        println!("the letter is: {}", letter);  
    }
}