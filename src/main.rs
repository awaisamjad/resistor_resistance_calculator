use std::collections::HashMap;
fn main() {
    let mut first_band_hashmap: HashMap<String, u32> = HashMap::new();
    first_band_hashmap.insert("Black".to_string(), 0);
    first_band_hashmap.insert("Brown".to_string(), 1);
    first_band_hashmap.insert("Red".to_string(), 2);
    first_band_hashmap.insert("Orange".to_string(), 3);
    first_band_hashmap.insert("Yellow".to_string(), 4);
    first_band_hashmap.insert("Green".to_string(), 5);
    first_band_hashmap.insert("Blue".to_string(), 6);
    first_band_hashmap.insert("Violet".to_string(), 7);
    first_band_hashmap.insert("Grey".to_string(), 8);
    first_band_hashmap.insert("White".to_string(), 9);

    let mut second_band_hashmap: HashMap<String, u32> = HashMap::new();
    second_band_hashmap.insert("Black".to_string(), 0);
    second_band_hashmap.insert("Brown".to_string(), 1);
    second_band_hashmap.insert("Red".to_string(), 2);
    second_band_hashmap.insert("Orange".to_string(), 3);
    second_band_hashmap.insert("Yellow".to_string(), 4);
    second_band_hashmap.insert("Green".to_string(), 5);
    second_band_hashmap.insert("Blue".to_string(), 6);
    second_band_hashmap.insert("Violet".to_string(), 7);
    second_band_hashmap.insert("Grey".to_string(), 8);
    second_band_hashmap.insert("White".to_string(), 9);

    let mut third_band_hashmap: HashMap<String, u32> = HashMap::new();
    third_band_hashmap.insert("Black".to_string(), 1);
    third_band_hashmap.insert("Brown".to_string(), 10);
    third_band_hashmap.insert("Red".to_string(), 100);
    third_band_hashmap.insert("Orange".to_string(), 1_000);
    third_band_hashmap.insert("Yellow".to_string(), 10_000);
    third_band_hashmap.insert("Green".to_string(), 100_000);
    third_band_hashmap.insert("Blue".to_string(), 1_000_000);
    third_band_hashmap.insert("Violet".to_string(), 10_000_000);
    third_band_hashmap.insert("Grey".to_string(), 100_000_000);
    third_band_hashmap.insert("White".to_string(), 1_000_000_000);

    let mut fourth_band_hashmap: HashMap<String, f32> = HashMap::new();
    fourth_band_hashmap.insert("Gold".to_string(), 1.05);
    fourth_band_hashmap.insert("Silver".to_string(), 1.1);
    fourth_band_hashmap.insert("None".to_string(), 1.2);

    let mut fifth_band_hashmap: HashMap<String, u8> = HashMap::new();
    let first_band = String::from("Red");
    let second_band = String::from("Black");
    let third_band = String::from("Green");
    let fourth_band = String::from("Silver");

    let mut first_band_value: u32 = 0;
    let mut second_band_value: u32 = 0;
    let mut third_band_value: u32 = 0;
    let mut fourth_band_value: f32 = 0.0;
    match first_band_hashmap.get(&first_band) {
        Some(value) => {
            first_band_value = *value * 10; //~ We multiply by 10 to make it 1 unit abpve the second band. if we didnt then we would have to turn the first and second values to chars, join them then turn back to u8
        }
        None => {
            println!("Key '{}' not found.", first_band);
        }
    }

    match second_band_hashmap.get(&second_band) {
        Some(value) => {
            second_band_value = *value;
        }
        None => {
            println!("Key '{}' not found.", second_band);
        }
    }

    match third_band_hashmap.get(&third_band) {
        Some(value) => {
            third_band_value = *value;
        }
        None => {
            println!("Key '{}' not found.", third_band);
        }
    }

    match fourth_band_hashmap.get(&fourth_band) {
        Some(value) => {
            fourth_band_value = *value;
        }
        None => {
            println!("Key '{}' not found.", fourth_band);
        }
    }
    let resistor_value = (first_band_value + second_band_value) * third_band_value;
    let upper_resistor_value = (resistor_value as f32) * fourth_band_value;
    let lower_resistor_value = (resistor_value as f32) * (1.0 - fourth_band_value);
    println!(
        "Resistor Value: {}  Upper Limit: {}  Lower Limit: {}",
        resistor_value,
        upper_resistor_value,
        lower_resistor_value
    );
}
