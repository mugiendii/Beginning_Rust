fn main() {
    hello_world();
    let sum = add(5, 10);
    println!("Sum: {}", sum);


    let remaining_litres = vend_litres(100.0, 30.5);
    println!("Remaining litres: {}", remaining_litres);
//block expression
    let _X: i32={
        let y = 10;
        let z = 20;
        y * z
    };
    println!("Value of X: {}", _X);


    let weight: f64 = 70.0;
    let height: f64 = 1.75;
    let bmi: f64= calculate_bmi( weight, height);
    println!("BMI: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, world!");
}

//parameterized function
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//VENDING FUNCTION
fn vend_litres(available_litres: f64, requested_litres: f64) -> f64 {
    if requested_litres <= available_litres {
        available_litres - requested_litres
    } else {
        println!("Insufficient litres available.");
        available_litres
    }
}

//BMI CALCULATOR FUNCTION
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}