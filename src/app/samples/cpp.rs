//! C++ code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Hello world
    ("#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}", "Hello, World!"),

    // 2. Template function
    ("template <typename T>\nT maximum(T a, T b) {\n    return a > b ? a : b;\n}", ""),

    // 3. Class
    ("class Counter {\npublic:\n    Counter() : count_(0) {}\n\n    void increment() {\n        ++count_;\n    }\n\n    int value() const {\n        return count_;\n    }\n\nprivate:\n    int count_;\n};", ""),

    // 4. Range-based for
    ("#include <vector>\n\nint sum_of_squares(const std::vector<int>& nums) {\n    int total = 0;\n    for (int n : nums) {\n        total += n * n;\n    }\n    return total;\n}", ""),

    // 5. Quicksort
    ("#include <vector>\n\nstd::vector<int> quick_sort(std::vector<int> arr) {\n    if (arr.size() <= 1) {\n        return arr;\n    }\n    int pivot = arr[0];\n    std::vector<int> left, right;\n    for (size_t i = 1; i < arr.size(); ++i) {\n        if (arr[i] < pivot) {\n            left.push_back(arr[i]);\n        } else {\n            right.push_back(arr[i]);\n        }\n    }\n    auto result = quick_sort(left);\n    result.push_back(pivot);\n    auto sorted_right = quick_sort(right);\n    result.insert(result.end(), sorted_right.begin(), sorted_right.end());\n    return result;\n}", ""),

    // 6. Smart pointers
    ("#include <memory>\n\nstruct Node {\n    int value;\n    std::unique_ptr<Node> next;\n\n    explicit Node(int v) : value(v), next(nullptr) {}\n};", ""),

    // 7. Lambda
    ("#include <vector>\n#include <algorithm>\n\nvoid sort_desc(std::vector<int>& v) {\n    std::sort(v.begin(), v.end(), [](int a, int b) {\n        return a > b;\n    });\n}", ""),

    // 8. Map
    ("#include <map>\n#include <string>\n#include <vector>\n\nstd::map<std::string, int> word_count(const std::vector<std::string>& words) {\n    std::map<std::string, int> counts;\n    for (const auto& word : words) {\n        counts[word]++;\n    }\n    return counts;\n}", ""),
];
