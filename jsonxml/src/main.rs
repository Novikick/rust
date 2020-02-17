mod convert;

fn main() {
    let data = r#" {
        "name": "John Doe",
        "age": 43,
        "address": {
          "street": "10 Downing Street",
          "city": "London"
        },
        "phones": [
          "+44 1234567",
          "+44 2345678"
        ]
      }
      "#;

    println!("{}", convert::convert_json_xml(data).unwrap());
}
