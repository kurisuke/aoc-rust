use crate::day::Day;
use serde_json::Value;

pub struct Day12 {}

fn visit_star1(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|v| visit_star1(v)).sum(),
        Value::Object(arr) => arr.values().map(|v| visit_star1(v)).sum(),
        _ => 0,
    }
}

fn visit_star2(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|v| visit_star2(v)).sum(),
        Value::Object(arr) => {
            if arr
                .values()
                .any(|v| v.is_string() && v.as_str().unwrap() == "red")
            {
                0
            } else {
                arr.values().map(|v| visit_star2(v)).sum()
            }
        }
        _ => 0,
    }
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let json: Value = serde_json::from_str(input).unwrap();
        let sum = visit_star1(&json);
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let json: Value = serde_json::from_str(input).unwrap();
        let sum = visit_star2(&json);
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day12 {};
        assert_eq!(d.star1(r#"[1,2,3]"#), "6");
        assert_eq!(d.star1(r#"{"a":2,"b":4}"#), "6");
        assert_eq!(d.star1(r#"[[[3]]]"#), "3");
        assert_eq!(d.star1(r#"{"a":{"b":4},"c":-1}"#), "3");
        assert_eq!(d.star1(r#"{"a":[-1,1]}"#), "0");
        assert_eq!(d.star1(r#"[-1,{"a":1}]"#), "0");
        assert_eq!(d.star1(r#"[]"#), "0");
        assert_eq!(d.star1(r#"{}"#), "0");
    }

    #[test]
    fn star2() {
        let d = Day12 {};
        assert_eq!(d.star2(r#"[1,2,3]"#), "6");
        assert_eq!(d.star2(r#"[1,{"c":"red","b":2},3]"#), "4");
        assert_eq!(d.star2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), "0");
        assert_eq!(d.star2(r#"[1,"red",5]"#), "6");
    }
}
