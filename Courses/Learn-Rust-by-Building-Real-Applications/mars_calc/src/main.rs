use std::io;

fn main() {
    // let a = 5;
    // let b = a;
    // println!("{}", a);
    // println!("{}", b);

    println!("Enter your weight (kg): ");
    let mut input = String::new();
    // let mut s = input;

    // let s1 = &input;
    // let s2 = &input;
    // println!("{} {}", s1, s2);

    // some_fn(&mut input);
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();

    println!("Hello, world!");
    println!("input: {}", input);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}

// fn some_fn(s: &String) {
fn some_fn(s: &mut String) {
    s.push_str("a");
}
