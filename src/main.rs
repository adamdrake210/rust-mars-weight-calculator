use std::io;

fn main() {
    println!("Please enter your weight in kg:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let weight: f32 = user_input.trim().parse().unwrap();
    
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
  (weight/ 9.81) * 3.71
}