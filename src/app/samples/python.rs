//! Python code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Quicksort
    ("def quicksort(arr):\n    if len(arr) <= 1:\n        return arr\n    pivot = arr[len(arr) // 2]\n    left = [x for x in arr if x < pivot]\n    middle = [x for x in arr if x == pivot]\n    right = [x for x in arr if x > pivot]\n    return quicksort(left) + middle + quicksort(right)", ""),

    // 2. Binary search
    ("def binary_search(arr, target):\n    lo, hi = 0, len(arr) - 1\n    while lo <= hi:\n        mid = (lo + hi) // 2\n        if arr[mid] == target:\n            return mid\n        elif arr[mid] < target:\n            lo = mid + 1\n        else:\n            hi = mid - 1\n    return -1", ""),

    // 3. FizzBuzz
    ("def fizzbuzz(n):\n    for i in range(1, n + 1):\n        if i % 15 == 0:\n            print(\"FizzBuzz\")\n        elif i % 3 == 0:\n            print(\"Fizz\")\n        elif i % 5 == 0:\n            print(\"Buzz\")\n        else:\n            print(i)", ""),

    // 4. Fibonacci with memoization
    ("def fibonacci(n, memo=None):\n    if memo is None:\n        memo = {}\n    if n in memo:\n        return memo[n]\n    if n <= 1:\n        return n\n    memo[n] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo)\n    return memo[n]", ""),

    // 5. Stack class
    ("class Stack:\n    def __init__(self):\n        self.items = []\n\n    def push(self, item):\n        self.items.append(item)\n\n    def pop(self):\n        return self.items.pop()\n\n    def peek(self):\n        return self.items[-1]\n\n    def is_empty(self):\n        return len(self.items) == 0", ""),

    // 6. Two sum
    ("def two_sum(nums, target):\n    seen = {}\n    for i, num in enumerate(nums):\n        complement = target - num\n        if complement in seen:\n            return [seen[complement], i]\n        seen[num] = i\n    return []", ""),

    // 7. Palindrome check
    ("def is_palindrome(s):\n    cleaned = [c.lower() for c in s if c.isalnum()]\n    return cleaned == cleaned[::-1]", ""),

    // 8. Merge sort
    ("def merge_sort(arr):\n    if len(arr) <= 1:\n        return arr\n    mid = len(arr) // 2\n    left = merge_sort(arr[:mid])\n    right = merge_sort(arr[mid:])\n    result = []\n    i = j = 0\n    while i < len(left) and j < len(right):\n        if left[i] <= right[j]:\n            result.append(left[i])\n            i += 1\n        else:\n            result.append(right[j])\n            j += 1\n    result.extend(left[i:])\n    result.extend(right[j:])\n    return result", ""),

    // 9. Word frequency counter
    ("def word_count(text):\n    counts = {}\n    for word in text.split():\n        word = word.lower()\n        counts[word] = counts.get(word, 0) + 1\n    return counts", ""),

    // 10. Decorator
    ("def repeat(times):\n    def decorator(func):\n        def wrapper(*args, **kwargs):\n            for _ in range(times):\n                func(*args, **kwargs)\n        return wrapper\n    return decorator\n\n@repeat(3)\ndef greet(name):\n    print(f\"Hello, {name}!\")", ""),
];
