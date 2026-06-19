//! JavaScript code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Quicksort
    ("function quickSort(arr) {\n  if (arr.length <= 1) return arr;\n  const pivot = arr[0];\n  const left = arr.slice(1).filter((x) => x < pivot);\n  const right = arr.slice(1).filter((x) => x >= pivot);\n  return [...quickSort(left), pivot, ...quickSort(right)];\n}", ""),

    // 2. Binary search
    ("function binarySearch(arr, target) {\n  let lo = 0;\n  let hi = arr.length - 1;\n  while (lo <= hi) {\n    const mid = Math.floor((lo + hi) / 2);\n    if (arr[mid] === target) return mid;\n    if (arr[mid] < target) lo = mid + 1;\n    else hi = mid - 1;\n  }\n  return -1;\n}", ""),

    // 3. FizzBuzz
    ("function fizzBuzz(n) {\n  for (let i = 1; i <= n; i++) {\n    if (i % 15 === 0) console.log(\"FizzBuzz\");\n    else if (i % 3 === 0) console.log(\"Fizz\");\n    else if (i % 5 === 0) console.log(\"Buzz\");\n    else console.log(i);\n  }\n}", ""),

    // 4. Fibonacci with memo
    ("function fibonacci(n, memo = {}) {\n  if (n in memo) return memo[n];\n  if (n <= 1) return n;\n  memo[n] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);\n  return memo[n];\n}", ""),

    // 5. Stack class
    ("class Stack {\n  constructor() {\n    this.items = [];\n  }\n\n  push(item) {\n    this.items.push(item);\n  }\n\n  pop() {\n    return this.items.pop();\n  }\n\n  peek() {\n    return this.items[this.items.length - 1];\n  }\n\n  get size() {\n    return this.items.length;\n  }\n}", ""),

    // 6. Two sum
    ("function twoSum(nums, target) {\n  const seen = new Map();\n  for (let i = 0; i < nums.length; i++) {\n    const complement = target - nums[i];\n    if (seen.has(complement)) {\n      return [seen.get(complement), i];\n    }\n    seen.set(nums[i], i);\n  }\n  return [];\n}", ""),

    // 7. Debounce closure
    ("function debounce(fn, delay) {\n  let timer = null;\n  return function (...args) {\n    clearTimeout(timer);\n    timer = setTimeout(() => fn.apply(this, args), delay);\n  };\n}", ""),

    // 8. Async / await
    ("async function fetchUser(id) {\n  const response = await fetch(`/api/users/${id}`);\n  if (!response.ok) {\n    throw new Error(`HTTP ${response.status}`);\n  }\n  return response.json();\n}", ""),

    // 9. Array pipeline
    ("const numbers = [1, 2, 3, 4, 5, 6];\nconst result = numbers\n  .filter((n) => n % 2 === 0)\n  .map((n) => n * n)\n  .reduce((sum, n) => sum + n, 0);\nconsole.log(result);", "56"),
];
