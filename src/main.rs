fn main() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32
   // println!("{}", cars_produced_per_minutes(6, 5) as i32); // to round the values we use i32
}

fn total_production(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;

    if speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }

    hours as f32 * 221.0 * success_rate * speed as f32
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    if speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }

    (hours as f32 * 221.0 * success_rate * speed as f32) / (60.0 * hours as f32)
}