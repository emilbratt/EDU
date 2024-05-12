use serde_json::Value;
use std::fs;

pub fn run() {
    let path = "./src/file_handling/read_json_file/types.json";

    let json_str = fs::read_to_string(path).unwrap();

    let data: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    string_value(&data);
    integer_value(&data);
    float_value(&data);
    bool_value(&data);
    null_value(&data);
    object_value(&data);
}

fn string_value(data: &serde_json::Value) {
    match data.get("is_string").and_then(Value::as_str) {
        Some(value) => println!("{}", value),
        None => panic!("not a string :("),
    }
}

fn integer_value(data: &serde_json::Value) {
    if let Some(value) = data.get("is_integer").and_then(Value::as_i64) {
        println!("Integer value: {}", value);
    } else {
        panic!("not an integer :(");
    }
}

fn float_value(data: &serde_json::Value) {
    if let Some(value) = data.get("is_float").and_then(Value::as_f64) {
        println!("float value: {}", value);
    } else {
        panic!("not a float :(");
    }
}

fn bool_value(data: &serde_json::Value) {
    if let Some(value) = data.get("is_bool").and_then(Value::as_bool) {
        println!("boolean value: {}", value);
    } else {
        panic!("not a boolean :(");
    }
}

fn null_value(data: &serde_json::Value) {
    match data.get("is_null").and_then(Value::as_null) {
        Some(_) => println!("it is null"),
        None => panic!("not a null :("),
    }
}

fn object_value(data: &serde_json::Value) {
    if let Some(value) = data.get("is_object").and_then(Value::as_object) {
        print!("object[name]: {}", value["name"]);
        print!(", object[age]: {}", value["age"]);
        println!(", object[city]: {}", value["city"]);

    } else {
        panic!("not an object :(");
    }
}
