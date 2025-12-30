pub const CODE_SAMPLES: &[&str] = &[
    "fn main() {\n    println!(\"Hello, world!\");\n}",
    "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
    "for i in 0..10 {\n    println!(\"{}\", i);\n}",
    "impl Point {\n    fn new(x: i32, y: i32) -> Self {\n        Point { x, y }\n    }\n}",
    "match value {\n    Some(x) => x,\n    None => 0\n}",
    "let numbers = vec![1, 2, 3];\nfor num in numbers {\n    println!(\"{}\", num);\n}",
    "if condition {\n    println!(\"True\");\n} else {\n    println!(\"False\");\n}",
    "fn factorial(n: u32) -> u32 {\n    match n {\n        0 => 1,\n        _ => n * factorial(n - 1)\n    }\n}",
    "struct Person {\n    name: String,\n    age: u32,\n}",
    "let vec = vec![1, 2, 3];\nlet doubled: Vec<_> = vec.iter()\n    .map(|x| x * 2)\n    .collect();",
];
