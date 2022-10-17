use crate::common::print_parts;
use ex::fs;
use serde_json::{Map, Value};
use std::{error::Error, io::BufReader};

fn sum_all_nums(val: &Value) -> i64 {
    match val {
        Value::Number(x) => x.as_i64().expect("Float ?"),
        Value::Array(arr) => arr.iter().map(sum_all_nums).sum(),
        Value::Object(obj) => obj.values().map(sum_all_nums).sum(),
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
    }
}

fn has_red_val(obj: &Map<String, Value>) -> bool {
    obj.values()
        .any(|val| *val == Value::String("red".to_owned()))
}

fn sum_all_nums_no_red(val: &Value) -> i64 {
    match val {
        Value::Number(x) => x.as_i64().expect("Float ?"),
        Value::Array(arr) => arr.iter().map(sum_all_nums_no_red).sum(),
        Value::Object(obj) => match has_red_val(obj) {
            true => 0,
            false => obj.values().map(sum_all_nums_no_red).sum(),
        },
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
    }
}

pub fn part1() -> Result<i64, Box<dyn Error>> {
    let buffer = BufReader::new(fs::File::open("data/d12.json")?);
    let content: Value = serde_json::from_reader(buffer)?;
    Ok(sum_all_nums(&content))
}

pub fn part2() -> Result<i64, Box<dyn Error>> {
    let buffer = BufReader::new(fs::File::open("data/d12.json")?);
    let content: Value = serde_json::from_reader(buffer)?;
    Ok(sum_all_nums_no_red(&content))
}

pub fn main() {
    print_parts(12, part1(), part2())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_sum_all_nums() {
        assert_eq!(sum_all_nums(&json!([1, 2, 3])), 6);
        assert_eq!(sum_all_nums(&json!({"a":2,"b":4})), 6);
        assert_eq!(sum_all_nums(&json!([[[3]]])), 3);
        assert_eq!(sum_all_nums(&json!({"a":{"b":4},"c":-1})), 3);
        assert_eq!(sum_all_nums(&json!({"a":[-1,1]})), 0);
        assert_eq!(sum_all_nums(&json!([-1,{"a":1}])), 0);
        assert_eq!(sum_all_nums(&json!([])), 0);
        assert_eq!(sum_all_nums(&json!({})), 0);
    }

    #[test]
    fn test_sum_has_red_val() {
        assert_eq!(
            has_red_val(&json!({"c":"blue","b":2}).as_object().unwrap()),
            false
        );
        assert_eq!(
            has_red_val(&json!({"c":"red","b":2}).as_object().unwrap()),
            true
        );
        assert_eq!(
            has_red_val(&json!({"d":"red","e":[1,2,3,4],"f":5}).as_object().unwrap()),
            true
        );
    }

    #[test]
    fn test_sum_all_nums_no_red() {
        assert_eq!(sum_all_nums_no_red(&json!([1, 2, 3])), 6);
        assert_eq!(sum_all_nums_no_red(&json!([1,{"c":"red","b":2},3])), 4);
        assert_eq!(
            sum_all_nums_no_red(&json!({"d":"red","e":[1,2,3,4],"f":5})),
            0
        );
        assert_eq!(sum_all_nums_no_red(&json!([1, "red", 5])), 6);
    }
}
