use std::collections::HashMap;
use std::io;
fn main() {
    //~ Create first band HashMap
    let first_band_hashmap: HashMap<String, u32> = HashMap::from([
        ("black".to_string(), 0),
        ("brown".to_string(), 1),
        ("red".to_string(), 2),
        ("orange".to_string(), 3),
        ("yellow".to_string(), 4),
        ("green".to_string(), 5),
        ("blue".to_string(), 6),
        ("violet".to_string(), 7),
        ("grey".to_string(), 8),
        ("white".to_string(), 9),
    ]);

    //~ Create second band HashMap
    let second_band_hashmap: HashMap<String, u32> = HashMap::from([
        ("black".to_string(), 0),
        ("brown".to_string(), 1),
        ("red".to_string(), 2),
        ("orange".to_string(), 3),
        ("yellow".to_string(), 4),
        ("green".to_string(), 5),
        ("blue".to_string(), 6),
        ("violet".to_string(), 7),
        ("grey".to_string(), 8),
        ("white".to_string(), 9),
    ]);

    //~ Create third band HashMap
    let third_band_hashmap: HashMap<String, u32> = HashMap::from([
        ("black".to_string(), 1),
        ("brown".to_string(), 10),
        ("red".to_string(), 100),
        ("orange".to_string(), 1000),
        ("yellow".to_string(), 10000),
        ("green".to_string(), 100000),
        ("blue".to_string(), 1000000),
        ("violet".to_string(), 10000000),
        ("grey".to_string(), 100000000),
        ("white".to_string(), 1000000000),
    ]);

    //~ Create fourth band HashMap
    let fourth_band_hashmap: HashMap<String, f32> = HashMap::from([
        ("silver".to_string(), 0.1),
        ("gold".to_string(), 0.05),
        ("black".to_string(), 0.2),
        ("brown".to_string(), 0.01),
        ("red".to_string(), 0.02),
        ("green".to_string(), 0.005),
        ("blue".to_string(), 0.0025),
        ("violet".to_string(), 0.001),
        ("grey".to_string(), 0.0005),
        ("white".to_string(), 0.0001),
    ]);

    let mut fifth_band_hashmap: HashMap<String, u8> = HashMap::new();

    println!("Please Enter Band Values");
    let first_band_value = get_first_band_value(first_band_hashmap);
    let second_band_value = get_second_band_value(second_band_hashmap);
    let third_band_value = get_third_band_value(third_band_hashmap);
    let fourth_band_value = get_fourth_band_value(fourth_band_hashmap);

    let resistor_value = ((first_band_value + second_band_value) * third_band_value) as f32;
    let upper_resistor_value = (resistor_value * fourth_band_value).round();
    let lower_resistor_value = resistor_value * (1.0 - (fourth_band_value - 1.0)).round();

    println!(
        "Resistor Value: {}  Upper Limit: {}  Lower Limit: {}",
        resistor_value,
        upper_resistor_value,
        lower_resistor_value
    );
}

fn get_first_band_value(first_band_hashmap: HashMap<String, u32>) -> u32 {
    loop {
        let mut first_band = String::new();
        //~ Read in the first band
        println!("Enter First Band Colour");
        io::stdin().read_line(&mut first_band).expect("Failed to Read First Band");
        let first_band = first_band.trim().to_string().to_lowercase();

        match first_band_hashmap.get(&first_band) {
            Some(value) => {
                let first_band_value = *value * 10; //~ We multiply by 10 to make it 1 unit above the second band. if we didnt then we would have to turn the first and second values to chars, join them then turn back to u8
                return first_band_value;
            }
            None => {
                println!("First Band Colur: '{}' not found.", first_band);
            }
        }
    }
}

fn get_second_band_value(second_band_hashmap: HashMap<String, u32>) -> u32 {
    loop {
        let mut second_band = String::new();
        //~ Read in the second band
        println!("Enter Second Band Colour");
        io::stdin().read_line(&mut second_band).expect("Failed to Read Second Band");
        let second_band = second_band.trim().to_string().to_lowercase();

        match second_band_hashmap.get(&second_band) {
            Some(value) => {
                let second_band_value = *value;
                return second_band_value;
            }
            None => {
                println!("Second Band Colour: '{}' not found.", second_band);
            }
        }
    }
}

fn get_third_band_value(third_band_hashmap: HashMap<String, u32>) -> u32 {
    loop {
        let mut third_band = String::new();
        //~ Read in the third band
        println!("Enter Third Band Colour");
        io::stdin().read_line(&mut third_band).expect("Failed to Read Third Band");
        let third_band = third_band.trim().to_string().to_lowercase();

        match third_band_hashmap.get(&third_band) {
            Some(value) => {
                let third_band_value = *value;
                return third_band_value;
            }
            None => {
                println!("Third Band Colour: '{}' not found.", third_band);
            }
        }
    }
}

fn get_fourth_band_value(fourth_band_hashmap: HashMap<String, f32>) -> f32 {
    loop {
        let mut fourth_band = String::new();
        //~ Read in the third band
        println!("Enter Fourth Band Colour");
        io::stdin().read_line(&mut fourth_band).expect("Failed to Read Fourth Band");
        let fourth_band = fourth_band.trim().to_string().to_lowercase();

        match fourth_band_hashmap.get(&fourth_band) {
            Some(value) => {
                let fourth_band_value = *value;
                return fourth_band_value;
            }
            None => {
                println!("Fourth Band Colour: '{}' not found.", fourth_band);
            }
        }
    }
}
