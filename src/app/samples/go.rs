//! Go code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Hello world
    ("package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello, World!\")\n}", "Hello, World!"),

    // 2. Quicksort
    ("func quickSort(arr []int) []int {\n    if len(arr) <= 1 {\n        return arr\n    }\n    pivot := arr[0]\n    var left, right []int\n    for _, x := range arr[1:] {\n        if x < pivot {\n            left = append(left, x)\n        } else {\n            right = append(right, x)\n        }\n    }\n    sorted := append(quickSort(left), pivot)\n    return append(sorted, quickSort(right)...)\n}", ""),

    // 3. Binary search
    ("func binarySearch(arr []int, target int) int {\n    lo, hi := 0, len(arr)-1\n    for lo <= hi {\n        mid := (lo + hi) / 2\n        if arr[mid] == target {\n            return mid\n        } else if arr[mid] < target {\n            lo = mid + 1\n        } else {\n            hi = mid - 1\n        }\n    }\n    return -1\n}", ""),

    // 4. Struct with methods
    ("type Rectangle struct {\n    Width  float64\n    Height float64\n}\n\nfunc (r Rectangle) Area() float64 {\n    return r.Width * r.Height\n}\n\nfunc (r Rectangle) Perimeter() float64 {\n    return 2 * (r.Width + r.Height)\n}", ""),

    // 5. Goroutines and channels
    ("func worker(id int, jobs <-chan int, results chan<- int) {\n    for j := range jobs {\n        results <- j * 2\n    }\n}\n\nfunc main() {\n    jobs := make(chan int, 5)\n    results := make(chan int, 5)\n    go worker(1, jobs, results)\n    for i := 1; i <= 3; i++ {\n        jobs <- i\n    }\n    close(jobs)\n}", ""),

    // 6. Maps
    ("func wordCount(words []string) map[string]int {\n    counts := make(map[string]int)\n    for _, w := range words {\n        counts[w]++\n    }\n    return counts\n}", ""),

    // 7. Error handling
    ("func divide(a, b float64) (float64, error) {\n    if b == 0 {\n        return 0, fmt.Errorf(\"division by zero\")\n    }\n    return a / b, nil\n}", ""),

    // 8. Interface
    ("type Shape interface {\n    Area() float64\n}\n\ntype Circle struct {\n    Radius float64\n}\n\nfunc (c Circle) Area() float64 {\n    return 3.14159 * c.Radius * c.Radius\n}", ""),
];
