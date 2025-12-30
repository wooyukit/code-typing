pub const CODE_SAMPLES: &[&str] = &[
    // Basic functions
    "fn main() {\n    println!(\"Hello, world!\");\n}",
    
    "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
    
    "fn greet(name: &str) -> String {\n    format!(\"Hello, {}!\", name)\n}",
    
    // Loops
    "for i in 0..10 {\n    println!(\"{}\", i);\n}",
    
    "let mut count = 0;\nwhile count < 5 {\n    println!(\"count: {}\", count);\n    count += 1;\n}",
    
    "loop {\n    println!(\"forever\");\n    break;\n}",
    
    // Structs and impl
    "struct Point {\n    x: i32,\n    y: i32,\n}",
    
    "struct Rectangle {\n    width: u32,\n    height: u32,\n}\n\nimpl Rectangle {\n    fn area(&self) -> u32 {\n        self.width * self.height\n    }\n}",
    
    "impl Point {\n    fn new(x: i32, y: i32) -> Self {\n        Point { x, y }\n    }\n\n    fn distance(&self, other: &Point) -> f64 {\n        let dx = (self.x - other.x) as f64;\n        let dy = (self.y - other.y) as f64;\n        (dx * dx + dy * dy).sqrt()\n    }\n}",
    
    // Pattern matching
    "match value {\n    Some(x) => x,\n    None => 0,\n}",
    
    "match number {\n    1 => println!(\"One\"),\n    2 | 3 => println!(\"Two or Three\"),\n    4..=10 => println!(\"Four to Ten\"),\n    _ => println!(\"Other\"),\n}",
    
    "let result = match color {\n    \"red\" => Color::Red,\n    \"blue\" => Color::Blue,\n    \"green\" => Color::Green,\n    _ => Color::Default,\n};",
    
    // Enums
    "enum Color {\n    Red,\n    Green,\n    Blue,\n    Rgb(u8, u8, u8),\n}",
    
    "enum Message {\n    Quit,\n    Move { x: i32, y: i32 },\n    Write(String),\n    ChangeColor(i32, i32, i32),\n}",
    
    // Option and Result
    "fn divide(a: f64, b: f64) -> Option<f64> {\n    if b == 0.0 {\n        None\n    } else {\n        Some(a / b)\n    }\n}",
    
    "fn parse_number(s: &str) -> Result<i32, String> {\n    s.parse().map_err(|_| \"Invalid number\".to_string())\n}",
    
    // Vectors and iterators
    "let numbers = vec![1, 2, 3];\nfor num in numbers {\n    println!(\"{}\", num);\n}",
    
    "let vec = vec![1, 2, 3, 4, 5];\nlet doubled: Vec<_> = vec.iter()\n    .map(|x| x * 2)\n    .collect();",
    
    "let numbers = vec![1, 2, 3, 4, 5, 6];\nlet evens: Vec<_> = numbers.iter()\n    .filter(|x| *x % 2 == 0)\n    .collect();",
    
    "let sum: i32 = (1..=100)\n    .filter(|x| x % 2 == 0)\n    .sum();",
    
    // If/else
    "if condition {\n    println!(\"True\");\n} else {\n    println!(\"False\");\n}",
    
    "let status = if score >= 90 {\n    \"Excellent\"\n} else if score >= 70 {\n    \"Good\"\n} else {\n    \"Needs improvement\"\n};",
    
    // Recursion
    "fn factorial(n: u32) -> u32 {\n    match n {\n        0 | 1 => 1,\n        _ => n * factorial(n - 1),\n    }\n}",
    
    "fn fibonacci(n: u32) -> u32 {\n    match n {\n        0 => 0,\n        1 => 1,\n        _ => fibonacci(n - 1) + fibonacci(n - 2),\n    }\n}",
    
    // Closures
    "let add = |a, b| a + b;\nlet result = add(5, 3);",
    
    "let multiply = |x: i32, y: i32| -> i32 {\n    x * y\n};",
    
    "let mut counter = 0;\nlet mut increment = || {\n    counter += 1;\n    counter\n};",
    
    // Traits
    "trait Animal {\n    fn name(&self) -> &str;\n    fn speak(&self);\n}",
    
    "impl Animal for Dog {\n    fn name(&self) -> &str {\n        &self.name\n    }\n\n    fn speak(&self) {\n        println!(\"Woof!\");\n    }\n}",
    
    // Generics
    "fn largest<T: PartialOrd>(list: &[T]) -> &T {\n    let mut largest = &list[0];\n    for item in list {\n        if item > largest {\n            largest = item;\n        }\n    }\n    largest\n}",
    
    "struct Wrapper<T> {\n    value: T,\n}\n\nimpl<T> Wrapper<T> {\n    fn new(value: T) -> Self {\n        Wrapper { value }\n    }\n}",
    
    // Lifetimes
    "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {\n    if x.len() > y.len() {\n        x\n    } else {\n        y\n    }\n}",
    
    // Error handling
    "let file = File::open(\"hello.txt\")?;\nlet mut contents = String::new();\nfile.read_to_string(&mut contents)?;",
    
    "match result {\n    Ok(value) => println!(\"Success: {}\", value),\n    Err(e) => println!(\"Error: {}\", e),\n}",
    
    // String manipulation
    "let s1 = String::from(\"Hello\");\nlet s2 = s1.clone();\nprintln!(\"{} {}\", s1, s2);",
    
    "let hello = \"Hello\".to_string();\nlet world = \"World\";\nlet greeting = format!(\"{}, {}!\", hello, world);",
    
    // HashMaps
    "use std::collections::HashMap;\n\nlet mut scores = HashMap::new();\nscores.insert(\"Blue\", 10);\nscores.insert(\"Red\", 50);",
    
    "for (key, value) in &map {\n    println!(\"{}: {}\", key, value);\n}",
    
    // Modules
    "mod front_of_house {\n    pub mod hosting {\n        pub fn add_to_waitlist() {}\n    }\n}",
    
    // Tests
    "#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn it_works() {\n        assert_eq!(2 + 2, 4);\n    }\n}",
    
    // Async
    "async fn fetch_data() -> Result<String, Error> {\n    let response = client.get(url).await?;\n    let body = response.text().await?;\n    Ok(body)\n}",
    
    // Builder pattern
    "impl ServerBuilder {\n    pub fn new() -> Self {\n        ServerBuilder::default()\n    }\n\n    pub fn port(mut self, port: u16) -> Self {\n        self.port = port;\n        self\n    }\n\n    pub fn build(self) -> Server {\n        Server {\n            port: self.port,\n            host: self.host,\n        }\n    }\n}",
    
    // Longer examples
    "pub struct Config {\n    pub query: String,\n    pub filename: String,\n    pub case_sensitive: bool,\n}\n\nimpl Config {\n    pub fn new(args: &[String]) -> Result<Config, &str> {\n        if args.len() < 3 {\n            return Err(\"not enough arguments\");\n        }\n        let query = args[1].clone();\n        let filename = args[2].clone();\n        Ok(Config { query, filename, case_sensitive: true })\n    }\n}",
    
    "fn process_items(items: Vec<Item>) -> Vec<ProcessedItem> {\n    items\n        .into_iter()\n        .filter(|item| item.is_valid())\n        .map(|item| item.process())\n        .collect()\n}",
];
