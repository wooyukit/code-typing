//! Swift code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Greeting with interpolation
    ("func greet(name: String) -> String {\n    return \"Hello, \\(name)!\"\n}\n\nprint(greet(name: \"World\"))", "Hello, World!"),

    // 2. Quicksort
    ("func quickSort<T: Comparable>(_ arr: [T]) -> [T] {\n    guard arr.count > 1 else { return arr }\n    let pivot = arr[arr.count / 2]\n    let less = arr.filter { $0 < pivot }\n    let equal = arr.filter { $0 == pivot }\n    let greater = arr.filter { $0 > pivot }\n    return quickSort(less) + equal + quickSort(greater)\n}", ""),

    // 3. Struct
    ("struct Point {\n    var x: Double\n    var y: Double\n\n    func distance(to other: Point) -> Double {\n        let dx = x - other.x\n        let dy = y - other.y\n        return (dx * dx + dy * dy).squareRoot()\n    }\n}", ""),

    // 4. Enum with associated values
    ("enum Outcome<T> {\n    case success(T)\n    case failure(String)\n\n    var isSuccess: Bool {\n        switch self {\n        case .success:\n            return true\n        case .failure:\n            return false\n        }\n    }\n}", ""),

    // 5. Protocol
    ("protocol Shape {\n    var area: Double { get }\n}\n\nstruct Circle: Shape {\n    let radius: Double\n\n    var area: Double {\n        return Double.pi * radius * radius\n    }\n}", ""),

    // 6. Optionals and guard
    ("func firstName(from fullName: String?) -> String {\n    guard let name = fullName, !name.isEmpty else {\n        return \"Unknown\"\n    }\n    return name.split(separator: \" \").first.map(String.init) ?? name\n}", ""),

    // 7. Closures and higher-order functions
    ("let numbers = [1, 2, 3, 4, 5, 6]\nlet evenSquares = numbers\n    .filter { $0 % 2 == 0 }\n    .map { $0 * $0 }\nprint(evenSquares)", "[4, 16, 36]"),

    // 8. Extension
    ("extension String {\n    func isPalindrome() -> Bool {\n        let cleaned = self.lowercased().filter { $0.isLetter }\n        return cleaned == String(cleaned.reversed())\n    }\n}", ""),
];
