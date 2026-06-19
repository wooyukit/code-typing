//! Kotlin code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Hello world
    ("fun main() {\n    println(\"Hello, World!\")\n}", "Hello, World!"),

    // 2. Quicksort
    ("fun quickSort(arr: List<Int>): List<Int> {\n    if (arr.size <= 1) return arr\n    val pivot = arr.first()\n    val rest = arr.drop(1)\n    val left = rest.filter { it < pivot }\n    val right = rest.filter { it >= pivot }\n    return quickSort(left) + pivot + quickSort(right)\n}", ""),

    // 3. Data class
    ("data class Person(\n    val name: String,\n    val age: Int,\n) {\n    fun isAdult(): Boolean = age >= 18\n}", ""),

    // 4. When expression
    ("fun describe(x: Any): String = when (x) {\n    0 -> \"zero\"\n    is Int -> \"integer\"\n    is String -> \"string of length ${x.length}\"\n    else -> \"unknown\"\n}", ""),

    // 5. Extension function
    ("fun String.isPalindrome(): Boolean {\n    val cleaned = this.lowercase().filter { it.isLetterOrDigit() }\n    return cleaned == cleaned.reversed()\n}", ""),

    // 6. Null safety
    ("fun firstName(fullName: String?): String {\n    return fullName?.split(\" \")?.firstOrNull() ?: \"Unknown\"\n}", ""),

    // 7. Higher-order function
    ("fun <T> Iterable<T>.countMatching(predicate: (T) -> Boolean): Int {\n    var count = 0\n    for (item in this) {\n        if (predicate(item)) count++\n    }\n    return count\n}", ""),

    // 8. Sealed class
    ("sealed class Result<out T> {\n    data class Success<T>(val value: T) : Result<T>()\n    data class Error(val message: String) : Result<Nothing>()\n}", ""),
];
