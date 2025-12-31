pub const CODE_SAMPLES: &[&str] = &[
    // 1. QuickSort partition
    "fn partition<T: Ord>(arr: &mut [T]) -> usize {\n    let len = arr.len();\n    let pivot = len / 2;\n    arr.swap(pivot, len - 1);\n    let mut store = 0;\n    for i in 0..len - 1 {\n        if arr[i] < arr[len - 1] {\n            arr.swap(i, store);\n            store += 1;\n        }\n    }\n    arr.swap(store, len - 1);\n    store\n}",

    // 2. Binary Search
    "fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {\n    let mut left = 0;\n    let mut right = arr.len();\n    while left < right {\n        let mid = left + (right - left) / 2;\n        match arr[mid].cmp(target) {\n            std::cmp::Ordering::Equal => return Some(mid),\n            std::cmp::Ordering::Less => left = mid + 1,\n            std::cmp::Ordering::Greater => right = mid,\n        }\n    }\n    None\n}",

    // 3. FizzBuzz
    "fn fizzbuzz(n: u32) -> String {\n    match (n % 3, n % 5) {\n        (0, 0) => String::from(\"FizzBuzz\"),\n        (0, _) => String::from(\"Fizz\"),\n        (_, 0) => String::from(\"Buzz\"),\n        _ => n.to_string(),\n    }\n}\n\nfn main() {\n    for i in 1..=100 {\n        println!(\"{}\", fizzbuzz(i));\n    }\n}",

    // 4. Fibonacci memoized
    "fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {\n    if n <= 1 { return n; }\n    if let Some(&result) = memo.get(&n) {\n        return result;\n    }\n    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);\n    memo.insert(n, result);\n    result\n}\n\nfn fibonacci_iter(n: u64) -> u64 {\n    let (mut a, mut b) = (0, 1);\n    for _ in 0..n {\n        let temp = a;\n        a = b;\n        b = temp + b;\n    }\n    a\n}",

    // 5. Linked List node
    "struct Node<T> {\n    value: T,\n    next: Option<Box<Node<T>>>,\n}\n\nimpl<T> Node<T> {\n    fn new(value: T) -> Self {\n        Node { value, next: None }\n    }\n\n    fn append(&mut self, value: T) {\n        match &mut self.next {\n            Some(next) => next.append(value),\n            None => self.next = Some(Box::new(Node::new(value))),\n        }\n    }\n}",

    // 6. Stack
    "struct Stack<T> {\n    data: Vec<T>,\n}\n\nimpl<T> Stack<T> {\n    fn new() -> Self {\n        Stack { data: Vec::new() }\n    }\n\n    fn push(&mut self, item: T) {\n        self.data.push(item);\n    }\n\n    fn pop(&mut self) -> Option<T> {\n        self.data.pop()\n    }\n\n    fn peek(&self) -> Option<&T> {\n        self.data.last()\n    }\n\n    fn is_empty(&self) -> bool {\n        self.data.is_empty()\n    }\n}",

    // 7. Queue
    "struct Queue<T> {\n    data: VecDeque<T>,\n}\n\nimpl<T> Queue<T> {\n    fn new() -> Self {\n        Queue { data: VecDeque::new() }\n    }\n\n    fn enqueue(&mut self, item: T) {\n        self.data.push_back(item);\n    }\n\n    fn dequeue(&mut self) -> Option<T> {\n        self.data.pop_front()\n    }\n\n    fn front(&self) -> Option<&T> {\n        self.data.front()\n    }\n\n    fn is_empty(&self) -> bool {\n        self.data.is_empty()\n    }\n}",

    // 8. Binary Tree node
    "struct TreeNode<T> {\n    value: T,\n    left: Option<Box<TreeNode<T>>>,\n    right: Option<Box<TreeNode<T>>>,\n}\n\nimpl<T> TreeNode<T> {\n    fn new(value: T) -> Self {\n        TreeNode { value, left: None, right: None }\n    }\n\n    fn insert_left(&mut self, value: T) {\n        self.left = Some(Box::new(TreeNode::new(value)));\n    }\n\n    fn insert_right(&mut self, value: T) {\n        self.right = Some(Box::new(TreeNode::new(value)));\n    }\n}",

    // 9. Factorial
    "fn factorial(n: u64) -> u64 {\n    match n {\n        0 | 1 => 1,\n        _ => n * factorial(n - 1),\n    }\n}\n\nfn factorial_iter(n: u64) -> u64 {\n    (1..=n).product()\n}\n\nfn main() {\n    for i in 0..10 {\n        println!(\"{}! = {}\", i, factorial(i));\n    }\n}",

    // 10. Two Sum
    "fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {\n    let mut map = HashMap::new();\n    for (i, &num) in nums.iter().enumerate() {\n        let complement = target - num;\n        if let Some(&j) = map.get(&complement) {\n            return Some((j, i));\n        }\n        map.insert(num, i);\n    }\n    None\n}\n\nfn main() {\n    let nums = vec![2, 7, 11, 15];\n    println!(\"{:?}\", two_sum(&nums, 9));\n}",

    // 11. Valid Parentheses
    "fn is_valid(s: &str) -> bool {\n    let mut stack = Vec::new();\n    for ch in s.chars() {\n        match ch {\n            '(' | '[' | '{' => stack.push(ch),\n            ')' => if stack.pop() != Some('(') { return false; }\n            ']' => if stack.pop() != Some('[') { return false; }\n            '}' => if stack.pop() != Some('{') { return false; }\n            _ => {}\n        }\n    }\n    stack.is_empty()\n}",

    // 12. Reverse string
    "fn reverse_string(s: &str) -> String {\n    s.chars().rev().collect()\n}\n\nfn reverse_words(s: &str) -> String {\n    s.split_whitespace()\n        .rev()\n        .collect::<Vec<_>>()\n        .join(\" \")\n}\n\nfn main() {\n    let s = \"hello world\";\n    println!(\"{}\", reverse_string(s));\n    println!(\"{}\", reverse_words(s));\n}",

    // 13. Palindrome check
    "fn is_palindrome(s: &str) -> bool {\n    let chars: Vec<char> = s.chars()\n        .filter(|c| c.is_alphanumeric())\n        .map(|c| c.to_ascii_lowercase())\n        .collect();\n    chars.iter().eq(chars.iter().rev())\n}\n\nfn main() {\n    assert!(is_palindrome(\"A man a plan a canal Panama\"));\n    assert!(is_palindrome(\"racecar\"));\n    assert!(!is_palindrome(\"hello\"));\n    println!(\"All tests passed!\");\n}",

    // 14. Max subarray (Kadane)
    "fn max_subarray(nums: &[i32]) -> i32 {\n    let mut max_sum = nums[0];\n    let mut current = nums[0];\n    for &num in nums.iter().skip(1) {\n        current = num.max(current + num);\n        max_sum = max_sum.max(current);\n    }\n    max_sum\n}\n\nfn main() {\n    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];\n    println!(\"Max sum: {}\", max_subarray(&nums));\n}",

    // 15. Merge sorted arrays
    "fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {\n    let mut result = Vec::with_capacity(a.len() + b.len());\n    let (mut i, mut j) = (0, 0);\n    while i < a.len() && j < b.len() {\n        if a[i] <= b[j] {\n            result.push(a[i]);\n            i += 1;\n        } else {\n            result.push(b[j]);\n            j += 1;\n        }\n    }\n    result.extend_from_slice(&a[i..]);\n    result.extend_from_slice(&b[j..]);\n    result\n}",

    // 16. Count occurrences
    "fn count_chars(s: &str) -> HashMap<char, usize> {\n    let mut counts = HashMap::new();\n    for ch in s.chars() {\n        *counts.entry(ch).or_insert(0) += 1;\n    }\n    counts\n}\n\nfn main() {\n    let text = \"hello world\";\n    for (ch, count) in count_chars(text) {\n        println!(\"'{}': {}\", ch, count);\n    }\n}",

    // 17. GCD and LCM
    "fn gcd(a: u64, b: u64) -> u64 {\n    if b == 0 { a } else { gcd(b, a % b) }\n}\n\nfn lcm(a: u64, b: u64) -> u64 {\n    a / gcd(a, b) * b\n}\n\nfn main() {\n    println!(\"GCD(48, 18) = {}\", gcd(48, 18));\n    println!(\"LCM(4, 6) = {}\", lcm(4, 6));\n}",

    // 18. Prime check
    "fn is_prime(n: u64) -> bool {\n    if n < 2 { return false; }\n    if n == 2 { return true; }\n    if n % 2 == 0 { return false; }\n    let sqrt = (n as f64).sqrt() as u64;\n    for i in (3..=sqrt).step_by(2) {\n        if n % i == 0 { return false; }\n    }\n    true\n}\n\nfn primes_up_to(n: u64) -> Vec<u64> {\n    (2..=n).filter(|&x| is_prime(x)).collect()\n}",

    // 19. Bubble sort
    "fn bubble_sort<T: Ord>(arr: &mut [T]) {\n    let n = arr.len();\n    for i in 0..n {\n        let mut swapped = false;\n        for j in 0..n - 1 - i {\n            if arr[j] > arr[j + 1] {\n                arr.swap(j, j + 1);\n                swapped = true;\n            }\n        }\n        if !swapped { break; }\n    }\n}",

    // 20. Selection sort
    "fn selection_sort<T: Ord>(arr: &mut [T]) {\n    let n = arr.len();\n    for i in 0..n {\n        let mut min_idx = i;\n        for j in (i + 1)..n {\n            if arr[j] < arr[min_idx] {\n                min_idx = j;\n            }\n        }\n        arr.swap(i, min_idx);\n    }\n}",

    // 21. Insertion sort
    "fn insertion_sort<T: Ord + Clone>(arr: &mut [T]) {\n    for i in 1..arr.len() {\n        let key = arr[i].clone();\n        let mut j = i;\n        while j > 0 && arr[j - 1] > key {\n            arr[j] = arr[j - 1].clone();\n            j -= 1;\n        }\n        arr[j] = key;\n    }\n}",

    // 22. Power function
    "fn power(base: i64, exp: u32) -> i64 {\n    match exp {\n        0 => 1,\n        1 => base,\n        n if n % 2 == 0 => {\n            let half = power(base, n / 2);\n            half * half\n        }\n        n => base * power(base, n - 1),\n    }\n}\n\nfn main() {\n    println!(\"2^10 = {}\", power(2, 10));\n    println!(\"3^5 = {}\", power(3, 5));\n}",

    // 23. String anagram
    "fn is_anagram(s1: &str, s2: &str) -> bool {\n    let mut chars1: Vec<char> = s1.to_lowercase().chars().collect();\n    let mut chars2: Vec<char> = s2.to_lowercase().chars().collect();\n    chars1.sort();\n    chars2.sort();\n    chars1 == chars2\n}\n\nfn main() {\n    assert!(is_anagram(\"listen\", \"silent\"));\n    assert!(is_anagram(\"Astronomer\", \"Moon starer\"));\n    println!(\"All tests passed!\");\n}",

    // 24. Remove duplicates
    "fn remove_duplicates<T: Eq + Hash + Clone>(arr: &[T]) -> Vec<T> {\n    let mut seen = HashSet::new();\n    arr.iter()\n        .filter(|x| seen.insert((*x).clone()))\n        .cloned()\n        .collect()\n}\n\nfn main() {\n    let nums = vec![1, 2, 2, 3, 4, 4, 5];\n    println!(\"{:?}\", remove_duplicates(&nums));\n}",

    // 25. Matrix transpose
    "fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {\n    let rows = matrix.len();\n    let cols = matrix[0].len();\n    let mut result = vec![vec![0; rows]; cols];\n    for i in 0..rows {\n        for j in 0..cols {\n            result[j][i] = matrix[i][j];\n        }\n    }\n    result\n}",

    // 26. Find missing number
    "fn find_missing(nums: &[i32], n: i32) -> i32 {\n    let expected: i32 = (n * (n + 1)) / 2;\n    let actual: i32 = nums.iter().sum();\n    expected - actual\n}\n\nfn main() {\n    let nums = vec![1, 2, 4, 5, 6];\n    println!(\"Missing: {}\", find_missing(&nums, 6));\n}",

    // 27. Binary to decimal
    "fn binary_to_decimal(binary: &str) -> u32 {\n    u32::from_str_radix(binary, 2).unwrap_or(0)\n}\n\nfn decimal_to_binary(n: u32) -> String {\n    format!(\"{:b}\", n)\n}\n\nfn main() {\n    println!(\"1010 = {}\", binary_to_decimal(\"1010\"));\n    println!(\"10 = {}\", decimal_to_binary(10));\n}",

    // 28. Rotate array
    "fn rotate_right<T: Clone>(arr: &mut [T], k: usize) {\n    let n = arr.len();\n    let k = k % n;\n    arr.reverse();\n    arr[..k].reverse();\n    arr[k..].reverse();\n}\n\nfn main() {\n    let mut arr = vec![1, 2, 3, 4, 5];\n    rotate_right(&mut arr, 2);\n    println!(\"{:?}\", arr);\n}",

    // 29. Find duplicates
    "fn find_duplicates(nums: &[i32]) -> Vec<i32> {\n    let mut seen = HashSet::new();\n    let mut duplicates = Vec::new();\n    for &num in nums {\n        if !seen.insert(num) {\n            duplicates.push(num);\n        }\n    }\n    duplicates\n}\n\nfn main() {\n    let nums = vec![1, 2, 3, 2, 4, 3, 5];\n    println!(\"{:?}\", find_duplicates(&nums));\n}",

    // 30. Sum of digits
    "fn sum_of_digits(mut n: u32) -> u32 {\n    let mut sum = 0;\n    while n > 0 {\n        sum += n % 10;\n        n /= 10;\n    }\n    sum\n}\n\nfn digital_root(n: u32) -> u32 {\n    if n < 10 { n } else { digital_root(sum_of_digits(n)) }\n}\n\nfn main() {\n    println!(\"Sum of 12345: {}\", sum_of_digits(12345));\n}",

    // 31. Min and max
    "fn find_min_max(arr: &[i32]) -> Option<(i32, i32)> {\n    if arr.is_empty() { return None; }\n    let mut min = arr[0];\n    let mut max = arr[0];\n    for &num in arr.iter().skip(1) {\n        if num < min { min = num; }\n        if num > max { max = num; }\n    }\n    Some((min, max))\n}\n\nfn main() {\n    let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];\n    println!(\"{:?}\", find_min_max(&arr));\n}",

    // 32. Second largest
    "fn second_largest(arr: &[i32]) -> Option<i32> {\n    if arr.len() < 2 { return None; }\n    let mut first = i32::MIN;\n    let mut second = i32::MIN;\n    for &num in arr {\n        if num > first {\n            second = first;\n            first = num;\n        } else if num > second && num != first {\n            second = num;\n        }\n    }\n    if second == i32::MIN { None } else { Some(second) }\n}",

    // 33. Move zeros
    "fn move_zeros(arr: &mut [i32]) {\n    let mut write = 0;\n    for read in 0..arr.len() {\n        if arr[read] != 0 {\n            arr.swap(write, read);\n            write += 1;\n        }\n    }\n}\n\nfn main() {\n    let mut arr = vec![0, 1, 0, 3, 12];\n    move_zeros(&mut arr);\n    println!(\"{:?}\", arr);\n}",

    // 34. Reverse linked list
    "fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {\n    let mut prev = None;\n    let mut current = head;\n    while let Some(mut node) = current {\n        let next = node.next.take();\n        node.next = prev;\n        prev = Some(node);\n        current = next;\n    }\n    prev\n}",

    // 35. Depth first search
    "fn dfs(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {\n    let mut visited = HashSet::new();\n    let mut result = Vec::new();\n    let mut stack = vec![start];\n    while let Some(node) = stack.pop() {\n        if visited.insert(node) {\n            result.push(node);\n            if let Some(neighbors) = graph.get(&node) {\n                for &n in neighbors.iter().rev() {\n                    stack.push(n);\n                }\n            }\n        }\n    }\n    result\n}",

    // 36. Breadth first search
    "fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {\n    let mut visited = HashSet::new();\n    let mut result = Vec::new();\n    let mut queue = VecDeque::new();\n    queue.push_back(start);\n    visited.insert(start);\n    while let Some(node) = queue.pop_front() {\n        result.push(node);\n        if let Some(neighbors) = graph.get(&node) {\n            for &n in neighbors {\n                if visited.insert(n) {\n                    queue.push_back(n);\n                }\n            }\n        }\n    }\n    result\n}",

    // 37. Flatten nested array
    "fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {\n    nested.iter().flat_map(|v| v.clone()).collect()\n}\n\nfn main() {\n    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];\n    println!(\"{:?}\", flatten(&nested));\n}",

    // 38. Chunk array
    "fn chunk<T: Clone>(arr: &[T], size: usize) -> Vec<Vec<T>> {\n    arr.chunks(size).map(|c| c.to_vec()).collect()\n}\n\nfn main() {\n    let arr = vec![1, 2, 3, 4, 5, 6, 7];\n    println!(\"{:?}\", chunk(&arr, 3));\n}",

    // 39. Deep clone struct
    "#[derive(Clone, Debug)]\nstruct Person {\n    name: String,\n    age: u32,\n    address: Address,\n}\n\n#[derive(Clone, Debug)]\nstruct Address {\n    city: String,\n    zip: String,\n}\n\nfn main() {\n    let p1 = Person {\n        name: \"Alice\".into(),\n        age: 30,\n        address: Address { city: \"NYC\".into(), zip: \"10001\".into() },\n    };\n    let p2 = p1.clone();\n    println!(\"{:?}\", p2);\n}",

    // 40. Iterator methods
    "fn iterator_examples() {\n    let nums = vec![1, 2, 3, 4, 5];\n    \n    let sum: i32 = nums.iter().sum();\n    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();\n    let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();\n    let found = nums.iter().find(|&&x| x > 3);\n    let all_positive = nums.iter().all(|&x| x > 0);\n    let any_even = nums.iter().any(|&x| x % 2 == 0);\n    \n    println!(\"Sum: {}, Found: {:?}\", sum, found);\n}",

    // 41. Error handling
    "fn divide(a: f64, b: f64) -> Result<f64, String> {\n    if b == 0.0 {\n        Err(\"Cannot divide by zero\".to_string())\n    } else {\n        Ok(a / b)\n    }\n}\n\nfn main() {\n    match divide(10.0, 2.0) {\n        Ok(result) => println!(\"Result: {}\", result),\n        Err(e) => println!(\"Error: {}\", e),\n    }\n}",

    // 42. Option handling
    "fn find_user(id: u32) -> Option<String> {\n    match id {\n        1 => Some(\"Alice\".to_string()),\n        2 => Some(\"Bob\".to_string()),\n        _ => None,\n    }\n}\n\nfn main() {\n    let user = find_user(1).unwrap_or(\"Unknown\".to_string());\n    println!(\"User: {}\", user);\n    \n    if let Some(name) = find_user(2) {\n        println!(\"Found: {}\", name);\n    }\n}",

    // 43. Pattern matching
    "enum Shape {\n    Circle { radius: f64 },\n    Rectangle { width: f64, height: f64 },\n    Square { side: f64 },\n}\n\nfn area(shape: &Shape) -> f64 {\n    match shape {\n        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,\n        Shape::Rectangle { width, height } => width * height,\n        Shape::Square { side } => side * side,\n    }\n}",

    // 44. Traits
    "trait Drawable {\n    fn draw(&self);\n    fn area(&self) -> f64;\n}\n\nstruct Circle { radius: f64 }\n\nimpl Drawable for Circle {\n    fn draw(&self) {\n        println!(\"Drawing circle with radius {}\", self.radius);\n    }\n    \n    fn area(&self) -> f64 {\n        std::f64::consts::PI * self.radius * self.radius\n    }\n}",

    // 45. Generics
    "fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {\n    if list.is_empty() { return None; }\n    let mut largest = &list[0];\n    for item in list {\n        if item > largest {\n            largest = item;\n        }\n    }\n    Some(largest)\n}\n\nfn main() {\n    let nums = vec![34, 50, 25, 100, 65];\n    println!(\"Largest: {:?}\", largest(&nums));\n}",

    // 46. Closures
    "fn main() {\n    let add = |a, b| a + b;\n    let multiply = |a: i32, b: i32| -> i32 { a * b };\n    \n    let mut counter = 0;\n    let mut increment = || {\n        counter += 1;\n        counter\n    };\n    \n    println!(\"Sum: {}\", add(5, 3));\n    println!(\"Product: {}\", multiply(4, 2));\n    println!(\"Count: {}\", increment());\n}",

    // 47. Vec operations
    "fn vec_operations() {\n    let mut v = vec![1, 2, 3];\n    \n    v.push(4);\n    v.pop();\n    v.insert(0, 0);\n    v.remove(0);\n    v.extend([4, 5, 6]);\n    v.retain(|&x| x > 2);\n    v.sort();\n    v.reverse();\n    v.dedup();\n    \n    println!(\"{:?}\", v);\n}",

    // 48. String operations
    "fn string_operations() {\n    let mut s = String::from(\"hello\");\n    \n    s.push_str(\" world\");\n    s.push('!');\n    let upper = s.to_uppercase();\n    let replaced = s.replace(\"world\", \"rust\");\n    let trimmed = s.trim();\n    let split: Vec<&str> = s.split(' ').collect();\n    let contains = s.contains(\"world\");\n    \n    println!(\"{} | {} | {}\", upper, replaced, contains);\n}",

    // 49. HashMap operations
    "fn hashmap_operations() {\n    let mut map = HashMap::new();\n    \n    map.insert(\"a\", 1);\n    map.insert(\"b\", 2);\n    map.entry(\"c\").or_insert(3);\n    \n    if let Some(val) = map.get(\"a\") {\n        println!(\"a = {}\", val);\n    }\n    \n    for (key, value) in &map {\n        println!(\"{}: {}\", key, value);\n    }\n    \n    map.remove(\"b\");\n}",

    // 50. File I/O
    "use std::fs::File;\nuse std::io::{Read, Write};\n\nfn write_file(path: &str, content: &str) -> std::io::Result<()> {\n    let mut file = File::create(path)?;\n    file.write_all(content.as_bytes())?;\n    Ok(())\n}\n\nfn read_file(path: &str) -> std::io::Result<String> {\n    let mut file = File::open(path)?;\n    let mut content = String::new();\n    file.read_to_string(&mut content)?;\n    Ok(content)\n}",
];
