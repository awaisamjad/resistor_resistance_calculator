use std::collections::HashMap;
use std::io;
fn main() {
    //~ Create first band HashMap
    let mut first_band_hashmap: HashMap<String, u32> = HashMap::new();
    first_band_hashmap.insert("black".to_string(), 0);
    first_band_hashmap.insert("brown".to_string(), 1);
    first_band_hashmap.insert("red".to_string(), 2);
    first_band_hashmap.insert("orange".to_string(), 3);
    first_band_hashmap.insert("yellow".to_string(), 4);
    first_band_hashmap.insert("green".to_string(), 5);
    first_band_hashmap.insert("blue".to_string(), 6);
    first_band_hashmap.insert("violet".to_string(), 7);
    first_band_hashmap.insert("grey".to_string(), 8);
    first_band_hashmap.insert("white".to_string(), 9);

    //~ Create second band HashMap
    let mut second_band_hashmap: HashMap<String, u32> = HashMap::new();
    second_band_hashmap.insert("black".to_string(), 0);
    second_band_hashmap.insert("brown".to_string(), 1);
    second_band_hashmap.insert("red".to_string(), 2);
    second_band_hashmap.insert("orange".to_string(), 3);
    second_band_hashmap.insert("yellow".to_string(), 4);
    second_band_hashmap.insert("green".to_string(), 5);
    second_band_hashmap.insert("blue".to_string(), 6);
    second_band_hashmap.insert("violet".to_string(), 7);
    second_band_hashmap.insert("grey".to_string(), 8);
    second_band_hashmap.insert("white".to_string(), 9);

    //~ Create third band HashMap
    let mut third_band_hashmap: HashMap<String, u32> = HashMap::new();
    third_band_hashmap.insert("black".to_string(), 1);
    third_band_hashmap.insert("brown".to_string(), 10);
    third_band_hashmap.insert("red".to_string(), 100);
    third_band_hashmap.insert("orange".to_string(), 1_000);
    third_band_hashmap.insert("yellow".to_string(), 10_000);
    third_band_hashmap.insert("green".to_string(), 100_000);
    third_band_hashmap.insert("blue".to_string(), 1_000_000);
    third_band_hashmap.insert("violet".to_string(), 10_000_000);
    third_band_hashmap.insert("grey".to_string(), 100_000_000);
    third_band_hashmap.insert("white".to_string(), 1_000_000_000);

    //~ Create fourth band HashMap
    let mut fourth_band_hashmap: HashMap<String, f32> = HashMap::new();
    fourth_band_hashmap.insert("gold".to_string(), 1.05);
    fourth_band_hashmap.insert("silver".to_string(), 1.1);
    fourth_band_hashmap.insert("none".to_string(), 1.2);

    let mut fifth_band_hashmap: HashMap<String, u8> = HashMap::new();

    println!("Please Enter Band Values");
    loop {
        let mut first_band = String::new();
        let mut first_band_value: u32 = 0;
        //~ Read in the first band
        println!("Enter First Band Colour");
        io::stdin().read_line(&mut first_band).expect("Failed to Read First Band");
        let first_band = first_band.trim().to_string().to_lowercase();

        match first_band_hashmap.get(&first_band) {
            Some(value) => {
                first_band_value = *value * 10; //~ We multiply by 10 to make it 1 unit above the second band. if we didnt then we would have to turn the first and second values to chars, join them then turn back to u8
                break;
            }
            None => {
                println!("First Band Colur: '{}' not found.", first_band);
            }
        }
    }

    loop {
        let mut second_band = String::new();
        let mut second_band_value: u32 = 0;
        //~ Read in the second band
        println!("Enter Second Band Colour");
        io::stdin().read_line(&mut second_band).expect("Failed to Read Second Band");
        let second_band = second_band.trim().to_string().to_lowercase();

        match second_band_hashmap.get(&second_band) {
            Some(value) => {
                second_band_value = *value;
                break;
            }
            None => {
                println!("Second Band Colour: '{}' not found.", second_band);
            }
        }
    }
    loop {
        let mut third_band = String::new();
        let mut third_band_value: u32 = 0;
        //~ Read in the third band
        println!("Enter Third Band Colour");
        io::stdin().read_line(&mut third_band).expect("Failed to Read Third Band");
        let third_band = third_band.trim().to_string().to_lowercase();

        match third_band_hashmap.get(&third_band) {
            Some(value) => {
                third_band_value = *value;
                break;
            }
            None => {
                println!("Third Band Colour: '{}' not found.", third_band);
            }
        }
    }

    loop {
        let mut fourth_band = String::new();
        let mut fourth_band_value: u32 = 0;
        //~ Read in the third band
        println!("Enter Third Band Colour");
        io::stdin().read_line(&mut fourth_band).expect("Failed to Read Third Band");
        let fourth_band = fourth_band.trim().to_string().to_lowercase();

        match fourth_band_hashmap.get(&fourth_band) {
            Some(value) => {
                fourth_band_value = *value;
                break;
            }
            None => {
                println!("Fourth Band Colour: '{}' not found.", fourth_band);
            }
        }
    }
    
    let resistor_value = ((first_band_value + second_band_value) * third_band_value) as f32;
    let upper_resistor_value = resistor_value * fourth_band_value;
    let lower_resistor_value = resistor_value * (1.0 - (fourth_band_value - 1.0));

    println!(
        "Resistor Value: {}  Upper Limit: {}  Lower Limit: {}",
        resistor_value,
        upper_resistor_value,
        lower_resistor_value
    );

}
