//! C code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Hello world
    ("#include <stdio.h>\n\nint main(void) {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}", "Hello, World!"),

    // 2. Factorial
    ("int factorial(int n) {\n    if (n <= 1) {\n        return 1;\n    }\n    return n * factorial(n - 1);\n}", ""),

    // 3. Binary search
    ("int binary_search(int arr[], int n, int target) {\n    int lo = 0, hi = n - 1;\n    while (lo <= hi) {\n        int mid = lo + (hi - lo) / 2;\n        if (arr[mid] == target) {\n            return mid;\n        } else if (arr[mid] < target) {\n            lo = mid + 1;\n        } else {\n            hi = mid - 1;\n        }\n    }\n    return -1;\n}", ""),

    // 4. Swap with pointers
    ("void swap(int *a, int *b) {\n    int temp = *a;\n    *a = *b;\n    *b = temp;\n}", ""),

    // 5. Struct
    ("struct Point {\n    double x;\n    double y;\n};\n\ndouble distance_squared(struct Point a, struct Point b) {\n    double dx = a.x - b.x;\n    double dy = a.y - b.y;\n    return dx * dx + dy * dy;\n}", ""),

    // 6. String length
    ("size_t str_length(const char *s) {\n    size_t len = 0;\n    while (s[len] != '\\0') {\n        len++;\n    }\n    return len;\n}", ""),

    // 7. Bubble sort
    ("void bubble_sort(int arr[], int n) {\n    for (int i = 0; i < n - 1; i++) {\n        for (int j = 0; j < n - 1 - i; j++) {\n            if (arr[j] > arr[j + 1]) {\n                int tmp = arr[j];\n                arr[j] = arr[j + 1];\n                arr[j + 1] = tmp;\n            }\n        }\n    }\n}", ""),

    // 8. Macro and main
    ("#include <stdio.h>\n\n#define MAX(a, b) ((a) > (b) ? (a) : (b))\n\nint main(void) {\n    int x = 10, y = 20;\n    printf(\"Max: %d\\n\", MAX(x, y));\n    return 0;\n}", "Max: 20"),
];
