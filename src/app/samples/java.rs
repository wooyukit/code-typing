//! Java code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Hello world
    ("public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}", "Hello, World!"),

    // 2. Quicksort
    ("public static void quickSort(int[] arr, int lo, int hi) {\n    if (lo < hi) {\n        int pivot = arr[hi];\n        int i = lo - 1;\n        for (int j = lo; j < hi; j++) {\n            if (arr[j] < pivot) {\n                i++;\n                int tmp = arr[i];\n                arr[i] = arr[j];\n                arr[j] = tmp;\n            }\n        }\n        int tmp = arr[i + 1];\n        arr[i + 1] = arr[hi];\n        arr[hi] = tmp;\n        quickSort(arr, lo, i);\n        quickSort(arr, i + 2, hi);\n    }\n}", ""),

    // 3. Binary search
    ("public static int binarySearch(int[] arr, int target) {\n    int lo = 0, hi = arr.length - 1;\n    while (lo <= hi) {\n        int mid = lo + (hi - lo) / 2;\n        if (arr[mid] == target) {\n            return mid;\n        } else if (arr[mid] < target) {\n            lo = mid + 1;\n        } else {\n            hi = mid - 1;\n        }\n    }\n    return -1;\n}", ""),

    // 4. Generic class
    ("public class Box<T> {\n    private T value;\n\n    public Box(T value) {\n        this.value = value;\n    }\n\n    public T get() {\n        return value;\n    }\n\n    public void set(T value) {\n        this.value = value;\n    }\n}", ""),

    // 5. Interface and implementation
    ("interface Shape {\n    double area();\n}\n\nclass Circle implements Shape {\n    private final double radius;\n\n    public Circle(double radius) {\n        this.radius = radius;\n    }\n\n    @Override\n    public double area() {\n        return Math.PI * radius * radius;\n    }\n}", ""),

    // 6. Streams
    ("import java.util.List;\nimport java.util.stream.Collectors;\n\npublic List<Integer> evenSquares(List<Integer> nums) {\n    return nums.stream()\n        .filter(n -> n % 2 == 0)\n        .map(n -> n * n)\n        .collect(Collectors.toList());\n}", ""),

    // 7. HashMap
    ("import java.util.HashMap;\nimport java.util.Map;\n\npublic Map<String, Integer> wordCount(String[] words) {\n    Map<String, Integer> counts = new HashMap<>();\n    for (String word : words) {\n        counts.merge(word, 1, Integer::sum);\n    }\n    return counts;\n}", ""),

    // 8. Record
    ("public record Point(int x, int y) {\n    public double distanceTo(Point other) {\n        int dx = x - other.x();\n        int dy = y - other.y();\n        return Math.sqrt(dx * dx + dy * dy);\n    }\n}", ""),
];
