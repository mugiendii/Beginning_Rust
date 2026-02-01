//Structs 
//used to package related data together to create custom data types

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Payload {
    recharge_balance: f64,
    recharge_times: u32,
    address: String,
    control_code: String,
    card_id: String,
    datetime: String,
    total_consumption: u32,
    data_id: String,
    ee_alarm: bool,
    battery1_alarm: bool,
    water_temperature_alarm: bool,
    valve_status: u32,
    reverse_flow_alarm: bool,
    pipe_leakage_alarm: bool,
    over_range_alarm: bool,
    battery2_alarm: bool,
    pipe_burst_alarm: bool,
    empty_tube_alarm: bool,
    flow_rate: f64,
    water_temperature: f64,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let payload1 = Payload {
        recharge_balance: 100.50,
        recharge_times: 5,
        address: String::from("68753500109093"),
        control_code: String::from("81"),
        card_id: String::from("3500109093"),
        datetime: String::from("2026-01-31T01:33:28"),
        total_consumption: 0,
        data_id: String::from("9012"),
        ee_alarm: false,
        battery1_alarm: false,
        water_temperature_alarm: false,
        valve_status: 1,
        reverse_flow_alarm: false,
        pipe_leakage_alarm: false,
        over_range_alarm: false,
        battery2_alarm: false,
        pipe_burst_alarm: false,
        empty_tube_alarm: true,
        flow_rate: 1.5,
        water_temperature: 27.34,
    };

    println!("User: {}, Email: {}", user1.username, user1.email);
    println!("Payload Balance: {}, Temperature: {}", payload1.recharge_balance, payload1.water_temperature);



    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color - R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("Point - X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);
}


//tu[le structs]

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);    

