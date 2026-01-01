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

    // 51. Merge Sort
    "fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {\n    let len = arr.len();\n    if len <= 1 { return; }\n    let mid = len / 2;\n    merge_sort(&mut arr[..mid]);\n    merge_sort(&mut arr[mid..]);\n    let mut merged = arr.to_vec();\n    merge(&arr[..mid], &arr[mid..], &mut merged);\n    arr.clone_from_slice(&merged);\n}\n\nfn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {\n    let (mut i, mut j, mut k) = (0, 0, 0);\n    while i < left.len() && j < right.len() {\n        if left[i] <= right[j] {\n            result[k] = left[i].clone();\n            i += 1;\n        } else {\n            result[k] = right[j].clone();\n            j += 1;\n        }\n        k += 1;\n    }\n    while i < left.len() { result[k] = left[i].clone(); i += 1; k += 1; }\n    while j < right.len() { result[k] = right[j].clone(); j += 1; k += 1; }\n}",

    // 52. Rc and RefCell
    "use std::rc::Rc;\nuse std::cell::RefCell;\n\nstruct Node {\n    value: i32,\n    children: Vec<Rc<RefCell<Node>>>,\n}\n\nimpl Node {\n    fn new(value: i32) -> Rc<RefCell<Self>> {\n        Rc::new(RefCell::new(Node {\n            value,\n            children: Vec::new(),\n        }))\n    }\n\n    fn add_child(&mut self, child: Rc<RefCell<Node>>) {\n        self.children.push(child);\n    }\n}",

    // 53. Custom Iterator
    "struct Counter {\n    count: u32,\n    max: u32,\n}\n\nimpl Counter {\n    fn new(max: u32) -> Self {\n        Counter { count: 0, max }\n    }\n}\n\nimpl Iterator for Counter {\n    type Item = u32;\n\n    fn next(&mut self) -> Option<Self::Item> {\n        if self.count < self.max {\n            self.count += 1;\n            Some(self.count)\n        } else {\n            None\n        }\n    }\n}\n\nfn main() {\n    for n in Counter::new(5) {\n        println!(\"{}\", n);\n    }\n}",

    // 54. Builder Pattern
    "struct Request {\n    url: String,\n    method: String,\n    headers: Vec<(String, String)>,\n    body: Option<String>,\n}\n\nstruct RequestBuilder {\n    url: String,\n    method: String,\n    headers: Vec<(String, String)>,\n    body: Option<String>,\n}\n\nimpl RequestBuilder {\n    fn new(url: &str) -> Self {\n        RequestBuilder {\n            url: url.to_string(),\n            method: \"GET\".to_string(),\n            headers: Vec::new(),\n            body: None,\n        }\n    }\n\n    fn method(mut self, method: &str) -> Self {\n        self.method = method.to_string();\n        self\n    }\n\n    fn header(mut self, key: &str, value: &str) -> Self {\n        self.headers.push((key.to_string(), value.to_string()));\n        self\n    }\n\n    fn body(mut self, body: &str) -> Self {\n        self.body = Some(body.to_string());\n        self\n    }\n\n    fn build(self) -> Request {\n        Request {\n            url: self.url,\n            method: self.method,\n            headers: self.headers,\n            body: self.body,\n        }\n    }\n}",

    // 55. From and Into traits
    "struct Celsius(f64);\nstruct Fahrenheit(f64);\n\nimpl From<Celsius> for Fahrenheit {\n    fn from(c: Celsius) -> Self {\n        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)\n    }\n}\n\nimpl From<Fahrenheit> for Celsius {\n    fn from(f: Fahrenheit) -> Self {\n        Celsius((f.0 - 32.0) * 5.0 / 9.0)\n    }\n}\n\nfn main() {\n    let c = Celsius(100.0);\n    let f: Fahrenheit = c.into();\n    println!(\"100C = {}F\", f.0);\n}",

    // 56. Drop trait
    "struct Resource {\n    name: String,\n}\n\nimpl Resource {\n    fn new(name: &str) -> Self {\n        println!(\"Creating {}\", name);\n        Resource { name: name.to_string() }\n    }\n}\n\nimpl Drop for Resource {\n    fn drop(&mut self) {\n        println!(\"Dropping {}\", self.name);\n    }\n}\n\nfn main() {\n    let r1 = Resource::new(\"resource1\");\n    {\n        let r2 = Resource::new(\"resource2\");\n    }\n    println!(\"End of main\");\n}",

    // 57. Deref trait
    "use std::ops::Deref;\n\nstruct MyBox<T>(T);\n\nimpl<T> MyBox<T> {\n    fn new(x: T) -> MyBox<T> {\n        MyBox(x)\n    }\n}\n\nimpl<T> Deref for MyBox<T> {\n    type Target = T;\n\n    fn deref(&self) -> &Self::Target {\n        &self.0\n    }\n}\n\nfn main() {\n    let x = 5;\n    let y = MyBox::new(x);\n    assert_eq!(5, *y);\n}",

    // 58. AsRef trait
    "fn print_length<T: AsRef<str>>(s: T) {\n    println!(\"Length: {}\", s.as_ref().len());\n}\n\nfn main() {\n    let string = String::from(\"hello\");\n    let str_slice = \"world\";\n    \n    print_length(string);\n    print_length(str_slice);\n}",

    // 59. Cow (Clone on Write)
    "use std::borrow::Cow;\n\nfn remove_spaces(s: &str) -> Cow<str> {\n    if s.contains(' ') {\n        Cow::Owned(s.replace(' ', \"\"))\n    } else {\n        Cow::Borrowed(s)\n    }\n}\n\nfn main() {\n    let s1 = \"hello\";\n    let s2 = \"hello world\";\n    \n    println!(\"{}\", remove_spaces(s1));\n    println!(\"{}\", remove_spaces(s2));\n}",

    // 60. PhantomData
    "use std::marker::PhantomData;\n\nstruct Meters;\nstruct Feet;\n\nstruct Distance<T> {\n    value: f64,\n    _unit: PhantomData<T>,\n}\n\nimpl<T> Distance<T> {\n    fn new(value: f64) -> Self {\n        Distance { value, _unit: PhantomData }\n    }\n}\n\nfn main() {\n    let m: Distance<Meters> = Distance::new(100.0);\n    let f: Distance<Feet> = Distance::new(328.0);\n    println!(\"{}m, {}ft\", m.value, f.value);\n}",

    // 61. Newtype pattern
    "struct UserId(u64);\nstruct OrderId(u64);\n\nimpl UserId {\n    fn new(id: u64) -> Self {\n        UserId(id)\n    }\n\n    fn value(&self) -> u64 {\n        self.0\n    }\n}\n\nfn get_user_orders(user_id: UserId) -> Vec<OrderId> {\n    vec![OrderId(1), OrderId(2)]\n}\n\nfn main() {\n    let user = UserId::new(42);\n    let orders = get_user_orders(user);\n}",

    // 62. Type State pattern
    "struct Locked;\nstruct Unlocked;\n\nstruct Door<State> {\n    _state: std::marker::PhantomData<State>,\n}\n\nimpl Door<Locked> {\n    fn new() -> Self {\n        Door { _state: std::marker::PhantomData }\n    }\n\n    fn unlock(self) -> Door<Unlocked> {\n        println!(\"Door unlocked\");\n        Door { _state: std::marker::PhantomData }\n    }\n}\n\nimpl Door<Unlocked> {\n    fn lock(self) -> Door<Locked> {\n        println!(\"Door locked\");\n        Door { _state: std::marker::PhantomData }\n    }\n\n    fn open(&self) {\n        println!(\"Door opened\");\n    }\n}",

    // 63. Default trait
    "#[derive(Debug)]\nstruct Config {\n    host: String,\n    port: u16,\n    timeout: u64,\n}\n\nimpl Default for Config {\n    fn default() -> Self {\n        Config {\n            host: \"localhost\".to_string(),\n            port: 8080,\n            timeout: 30,\n        }\n    }\n}\n\nfn main() {\n    let config = Config::default();\n    let custom = Config { port: 3000, ..Default::default() };\n    println!(\"{:?}\", config);\n    println!(\"{:?}\", custom);\n}",

    // 64. Display trait
    "use std::fmt;\n\nstruct Point {\n    x: f64,\n    y: f64,\n}\n\nimpl fmt::Display for Point {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        write!(f, \"({}, {})\", self.x, self.y)\n    }\n}\n\nimpl fmt::Debug for Point {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        write!(f, \"Point {{ x: {}, y: {} }}\", self.x, self.y)\n    }\n}\n\nfn main() {\n    let p = Point { x: 3.0, y: 4.0 };\n    println!(\"{}\", p);\n    println!(\"{:?}\", p);\n}",

    // 65. PartialEq and Eq
    "#[derive(Debug)]\nstruct Person {\n    name: String,\n    age: u32,\n}\n\nimpl PartialEq for Person {\n    fn eq(&self, other: &Self) -> bool {\n        self.name == other.name\n    }\n}\n\nimpl Eq for Person {}\n\nfn main() {\n    let p1 = Person { name: \"Alice\".into(), age: 30 };\n    let p2 = Person { name: \"Alice\".into(), age: 25 };\n    println!(\"p1 == p2: {}\", p1 == p2);\n}",

    // 66. PartialOrd and Ord
    "#[derive(Debug, Eq, PartialEq)]\nstruct Version {\n    major: u32,\n    minor: u32,\n    patch: u32,\n}\n\nimpl PartialOrd for Version {\n    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {\n        Some(self.cmp(other))\n    }\n}\n\nimpl Ord for Version {\n    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n        (self.major, self.minor, self.patch)\n            .cmp(&(other.major, other.minor, other.patch))\n    }\n}\n\nfn main() {\n    let v1 = Version { major: 1, minor: 2, patch: 3 };\n    let v2 = Version { major: 1, minor: 3, patch: 0 };\n    println!(\"v1 < v2: {}\", v1 < v2);\n}",

    // 67. Hash trait
    "use std::collections::HashSet;\nuse std::hash::{Hash, Hasher};\n\n#[derive(Debug)]\nstruct Point {\n    x: i32,\n    y: i32,\n}\n\nimpl Hash for Point {\n    fn hash<H: Hasher>(&self, state: &mut H) {\n        self.x.hash(state);\n        self.y.hash(state);\n    }\n}\n\nimpl PartialEq for Point {\n    fn eq(&self, other: &Self) -> bool {\n        self.x == other.x && self.y == other.y\n    }\n}\n\nimpl Eq for Point {}\n\nfn main() {\n    let mut set = HashSet::new();\n    set.insert(Point { x: 1, y: 2 });\n    set.insert(Point { x: 3, y: 4 });\n}",

    // 68. Index trait
    "use std::ops::Index;\n\nstruct Matrix {\n    data: Vec<Vec<i32>>,\n}\n\nimpl Index<(usize, usize)> for Matrix {\n    type Output = i32;\n\n    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {\n        &self.data[row][col]\n    }\n}\n\nfn main() {\n    let m = Matrix {\n        data: vec![vec![1, 2], vec![3, 4]],\n    };\n    println!(\"m[1,1] = {}\", m[(1, 1)]);\n}",

    // 69. Add trait
    "use std::ops::Add;\n\n#[derive(Debug, Clone, Copy)]\nstruct Vector2 {\n    x: f64,\n    y: f64,\n}\n\nimpl Add for Vector2 {\n    type Output = Self;\n\n    fn add(self, other: Self) -> Self::Output {\n        Vector2 {\n            x: self.x + other.x,\n            y: self.y + other.y,\n        }\n    }\n}\n\nfn main() {\n    let v1 = Vector2 { x: 1.0, y: 2.0 };\n    let v2 = Vector2 { x: 3.0, y: 4.0 };\n    let v3 = v1 + v2;\n    println!(\"{:?}\", v3);\n}",

    // 70. Mul trait
    "use std::ops::Mul;\n\n#[derive(Debug, Clone, Copy)]\nstruct Vector2 {\n    x: f64,\n    y: f64,\n}\n\nimpl Mul<f64> for Vector2 {\n    type Output = Self;\n\n    fn mul(self, scalar: f64) -> Self::Output {\n        Vector2 {\n            x: self.x * scalar,\n            y: self.y * scalar,\n        }\n    }\n}\n\nfn main() {\n    let v = Vector2 { x: 2.0, y: 3.0 };\n    let scaled = v * 2.5;\n    println!(\"{:?}\", scaled);\n}",

    // 71. BinaryHeap
    "use std::collections::BinaryHeap;\n\nfn main() {\n    let mut heap = BinaryHeap::new();\n    heap.push(3);\n    heap.push(1);\n    heap.push(4);\n    heap.push(1);\n    heap.push(5);\n\n    while let Some(val) = heap.pop() {\n        println!(\"{}\", val);\n    }\n}",

    // 72. BTreeMap
    "use std::collections::BTreeMap;\n\nfn main() {\n    let mut map = BTreeMap::new();\n    map.insert(\"c\", 3);\n    map.insert(\"a\", 1);\n    map.insert(\"b\", 2);\n\n    for (key, value) in &map {\n        println!(\"{}: {}\", key, value);\n    }\n\n    if let Some(range) = map.range(\"a\"..\"c\").next() {\n        println!(\"First in range: {:?}\", range);\n    }\n}",

    // 73. VecDeque
    "use std::collections::VecDeque;\n\nfn main() {\n    let mut deque = VecDeque::new();\n    deque.push_back(1);\n    deque.push_back(2);\n    deque.push_front(0);\n\n    println!(\"Front: {:?}\", deque.front());\n    println!(\"Back: {:?}\", deque.back());\n\n    while let Some(val) = deque.pop_front() {\n        println!(\"{}\", val);\n    }\n}",

    // 74. LinkedList
    "use std::collections::LinkedList;\n\nfn main() {\n    let mut list = LinkedList::new();\n    list.push_back(1);\n    list.push_back(2);\n    list.push_front(0);\n\n    for val in &list {\n        println!(\"{}\", val);\n    }\n\n    let mut list2 = LinkedList::new();\n    list2.push_back(3);\n    list.append(&mut list2);\n}",

    // 75. Entry API
    "use std::collections::HashMap;\n\nfn word_count(text: &str) -> HashMap<String, u32> {\n    let mut counts = HashMap::new();\n    for word in text.split_whitespace() {\n        let word = word.to_lowercase();\n        *counts.entry(word).or_insert(0) += 1;\n    }\n    counts\n}\n\nfn main() {\n    let text = \"hello world hello rust world\";\n    for (word, count) in word_count(text) {\n        println!(\"{}: {}\", word, count);\n    }\n}",

    // 76. Lifetime annotations
    "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {\n    if x.len() > y.len() { x } else { y }\n}\n\nstruct Excerpt<'a> {\n    part: &'a str,\n}\n\nimpl<'a> Excerpt<'a> {\n    fn level(&self) -> i32 {\n        3\n    }\n\n    fn announce(&self, announcement: &str) -> &str {\n        println!(\"Attention: {}\", announcement);\n        self.part\n    }\n}\n\nfn main() {\n    let s1 = \"long string\";\n    let s2 = \"short\";\n    println!(\"Longest: {}\", longest(s1, s2));\n}",

    // 77. Static lifetime
    "static GREETING: &str = \"Hello, World!\";\n\nfn get_static_str() -> &'static str {\n    \"I live forever\"\n}\n\nfn main() {\n    println!(\"{}\", GREETING);\n    let s: &'static str = get_static_str();\n    println!(\"{}\", s);\n}",

    // 78. Box smart pointer
    "enum List {\n    Cons(i32, Box<List>),\n    Nil,\n}\n\nuse List::{Cons, Nil};\n\nfn main() {\n    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));\n\n    fn sum(list: &List) -> i32 {\n        match list {\n            Cons(val, next) => val + sum(next),\n            Nil => 0,\n        }\n    }\n\n    println!(\"Sum: {}\", sum(&list));\n}",

    // 79. Arc for thread safety
    "use std::sync::Arc;\nuse std::thread;\n\nfn main() {\n    let data = Arc::new(vec![1, 2, 3, 4, 5]);\n    let mut handles = vec![];\n\n    for i in 0..3 {\n        let data = Arc::clone(&data);\n        let handle = thread::spawn(move || {\n            println!(\"Thread {}: {:?}\", i, data);\n        });\n        handles.push(handle);\n    }\n\n    for handle in handles {\n        handle.join().unwrap();\n    }\n}",

    // 80. Mutex
    "use std::sync::Mutex;\n\nfn main() {\n    let m = Mutex::new(5);\n\n    {\n        let mut num = m.lock().unwrap();\n        *num = 6;\n    }\n\n    println!(\"m = {:?}\", m);\n}",

    // 81. RwLock
    "use std::sync::RwLock;\n\nfn main() {\n    let lock = RwLock::new(5);\n\n    {\n        let r1 = lock.read().unwrap();\n        let r2 = lock.read().unwrap();\n        println!(\"Readers: {} and {}\", *r1, *r2);\n    }\n\n    {\n        let mut w = lock.write().unwrap();\n        *w += 1;\n        println!(\"Writer: {}\", *w);\n    }\n}",

    // 82. Channel communication
    "use std::sync::mpsc;\nuse std::thread;\n\nfn main() {\n    let (tx, rx) = mpsc::channel();\n\n    thread::spawn(move || {\n        let vals = vec![\"hi\", \"from\", \"the\", \"thread\"];\n        for val in vals {\n            tx.send(val).unwrap();\n        }\n    });\n\n    for received in rx {\n        println!(\"Got: {}\", received);\n    }\n}",

    // 83. Multiple producers
    "use std::sync::mpsc;\nuse std::thread;\n\nfn main() {\n    let (tx, rx) = mpsc::channel();\n    let tx1 = tx.clone();\n\n    thread::spawn(move || {\n        tx.send(\"from tx\").unwrap();\n    });\n\n    thread::spawn(move || {\n        tx1.send(\"from tx1\").unwrap();\n    });\n\n    for received in rx {\n        println!(\"Got: {}\", received);\n    }\n}",

    // 84. Thread spawn and join
    "use std::thread;\nuse std::time::Duration;\n\nfn main() {\n    let handle = thread::spawn(|| {\n        for i in 1..10 {\n            println!(\"Thread: {}\", i);\n            thread::sleep(Duration::from_millis(1));\n        }\n    });\n\n    for i in 1..5 {\n        println!(\"Main: {}\", i);\n        thread::sleep(Duration::from_millis(1));\n    }\n\n    handle.join().unwrap();\n}",

    // 85. Move closure with thread
    "use std::thread;\n\nfn main() {\n    let v = vec![1, 2, 3];\n\n    let handle = thread::spawn(move || {\n        println!(\"Vector: {:?}\", v);\n    });\n\n    handle.join().unwrap();\n}",

    // 86. Custom error type
    "#[derive(Debug)]\nenum AppError {\n    IoError(std::io::Error),\n    ParseError(std::num::ParseIntError),\n    Custom(String),\n}\n\nimpl std::fmt::Display for AppError {\n    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n        match self {\n            AppError::IoError(e) => write!(f, \"IO error: {}\", e),\n            AppError::ParseError(e) => write!(f, \"Parse error: {}\", e),\n            AppError::Custom(s) => write!(f, \"Error: {}\", s),\n        }\n    }\n}\n\nimpl std::error::Error for AppError {}",

    // 87. Question mark operator
    "fn read_number(s: &str) -> Result<i32, std::num::ParseIntError> {\n    let n = s.trim().parse::<i32>()?;\n    Ok(n * 2)\n}\n\nfn main() {\n    match read_number(\"42\") {\n        Ok(n) => println!(\"Result: {}\", n),\n        Err(e) => println!(\"Error: {}\", e),\n    }\n}",

    // 88. Option combinators
    "fn main() {\n    let some_num = Some(5);\n    let none_num: Option<i32> = None;\n\n    let mapped = some_num.map(|x| x * 2);\n    let and_then = some_num.and_then(|x| Some(x + 1));\n    let unwrap_or = none_num.unwrap_or(0);\n    let unwrap_or_else = none_num.unwrap_or_else(|| 42);\n    let ok_or = some_num.ok_or(\"error\");\n\n    println!(\"{:?}, {:?}, {}, {}\", mapped, and_then, unwrap_or, unwrap_or_else);\n}",

    // 89. Result combinators
    "fn main() {\n    let ok: Result<i32, &str> = Ok(5);\n    let err: Result<i32, &str> = Err(\"error\");\n\n    let mapped = ok.map(|x| x * 2);\n    let map_err = err.map_err(|e| format!(\"Error: {}\", e));\n    let and_then = ok.and_then(|x| Ok(x + 1));\n    let unwrap_or = err.unwrap_or(0);\n\n    println!(\"{:?}, {:?}, {:?}, {}\", mapped, map_err, and_then, unwrap_or);\n}",

    // 90. Slice patterns
    "fn describe_slice(slice: &[i32]) {\n    match slice {\n        [] => println!(\"empty\"),\n        [x] => println!(\"single: {}\", x),\n        [x, y] => println!(\"pair: {}, {}\", x, y),\n        [first, .., last] => println!(\"first: {}, last: {}\", first, last),\n    }\n}\n\nfn main() {\n    describe_slice(&[]);\n    describe_slice(&[1]);\n    describe_slice(&[1, 2]);\n    describe_slice(&[1, 2, 3, 4, 5]);\n}",

    // 91. If let and while let
    "fn main() {\n    let optional = Some(7);\n\n    if let Some(x) = optional {\n        println!(\"Got: {}\", x);\n    }\n\n    let mut stack = vec![1, 2, 3];\n\n    while let Some(top) = stack.pop() {\n        println!(\"{}\", top);\n    }\n}",

    // 92. Let else
    "fn get_count(s: &str) -> Option<u32> {\n    let Some(count) = s.parse::<u32>().ok() else {\n        return None;\n    };\n    Some(count * 2)\n}\n\nfn main() {\n    println!(\"{:?}\", get_count(\"42\"));\n    println!(\"{:?}\", get_count(\"abc\"));\n}",

    // 93. Matches macro
    "fn main() {\n    let foo = 'f';\n    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));\n\n    let bar = Some(4);\n    assert!(matches!(bar, Some(x) if x > 2));\n\n    let v = vec![1, 2, 3];\n    assert!(matches!(v.as_slice(), [1, ..]));\n}",

    // 94. Collect into different types
    "fn main() {\n    let v: Vec<i32> = (0..5).collect();\n    let s: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();\n    let set: std::collections::HashSet<i32> = v.iter().cloned().collect();\n    let map: std::collections::HashMap<i32, i32> = v.iter().map(|&x| (x, x * 2)).collect();\n\n    println!(\"{:?}\", v);\n    println!(\"{}\", s);\n    println!(\"{:?}\", set);\n    println!(\"{:?}\", map);\n}",

    // 95. Enumerate and zip
    "fn main() {\n    let v = vec!['a', 'b', 'c'];\n\n    for (i, c) in v.iter().enumerate() {\n        println!(\"{}: {}\", i, c);\n    }\n\n    let nums = vec![1, 2, 3];\n    let chars = vec!['a', 'b', 'c'];\n\n    for (n, c) in nums.iter().zip(chars.iter()) {\n        println!(\"{} -> {}\", n, c);\n    }\n}",

    // 96. Take and skip
    "fn main() {\n    let v: Vec<i32> = (0..10).collect();\n\n    let first_five: Vec<i32> = v.iter().take(5).cloned().collect();\n    let skip_three: Vec<i32> = v.iter().skip(3).cloned().collect();\n    let take_while: Vec<i32> = v.iter().take_while(|&&x| x < 5).cloned().collect();\n    let skip_while: Vec<i32> = v.iter().skip_while(|&&x| x < 5).cloned().collect();\n\n    println!(\"{:?}\", first_five);\n    println!(\"{:?}\", skip_three);\n    println!(\"{:?}\", take_while);\n    println!(\"{:?}\", skip_while);\n}",

    // 97. Fold and reduce
    "fn main() {\n    let nums = vec![1, 2, 3, 4, 5];\n\n    let sum: i32 = nums.iter().fold(0, |acc, x| acc + x);\n    let product: i32 = nums.iter().fold(1, |acc, x| acc * x);\n    let concat: String = nums.iter().fold(String::new(), |acc, x| acc + &x.to_string());\n\n    let max = nums.iter().cloned().reduce(|a, b| if a > b { a } else { b });\n\n    println!(\"Sum: {}, Product: {}\", sum, product);\n    println!(\"Concat: {}, Max: {:?}\", concat, max);\n}",

    // 98. Partition and group
    "fn main() {\n    let nums: Vec<i32> = (0..10).collect();\n\n    let (evens, odds): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&&x| x % 2 == 0);\n\n    println!(\"Evens: {:?}\", evens);\n    println!(\"Odds: {:?}\", odds);\n\n    let words = vec![\"apple\", \"banana\", \"apricot\", \"blueberry\"];\n    let grouped: std::collections::HashMap<char, Vec<&&str>> = words\n        .iter()\n        .fold(std::collections::HashMap::new(), |mut acc, word| {\n            acc.entry(word.chars().next().unwrap()).or_default().push(word);\n            acc\n        });\n    println!(\"{:?}\", grouped);\n}",

    // 99. Peekable iterator
    "fn main() {\n    let v = vec![1, 2, 3, 4, 5];\n    let mut iter = v.iter().peekable();\n\n    while let Some(&x) = iter.next() {\n        if let Some(&&next) = iter.peek() {\n            println!(\"{} followed by {}\", x, next);\n        } else {\n            println!(\"{} is last\", x);\n        }\n    }\n}",

    // 100. Flatten and flat_map
    "fn main() {\n    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];\n    let flat: Vec<i32> = nested.iter().flatten().cloned().collect();\n    println!(\"{:?}\", flat);\n\n    let words = vec![\"hello\", \"world\"];\n    let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();\n    println!(\"{:?}\", chars);\n\n    let options = vec![Some(1), None, Some(3)];\n    let values: Vec<i32> = options.into_iter().flatten().collect();\n    println!(\"{:?}\", values);\n}",
];
