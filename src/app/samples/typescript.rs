//! TypeScript code samples: (code, expected_output). Empty output suppresses the output panel.
pub const SAMPLES: &[(&str, &str)] = &[
    // 1. Interfaces
    ("interface User {\n  id: number;\n  name: string;\n  email?: string;\n}\n\nfunction greet(user: User): string {\n  return `Hello, ${user.name}`;\n}", ""),

    // 2. Generic functions
    ("function identity<T>(value: T): T {\n  return value;\n}\n\nfunction first<T>(items: T[]): T | undefined {\n  return items.length > 0 ? items[0] : undefined;\n}", ""),

    // 3. Generic class
    ("class Stack<T> {\n  private items: T[] = [];\n\n  push(item: T): void {\n    this.items.push(item);\n  }\n\n  pop(): T | undefined {\n    return this.items.pop();\n  }\n\n  get size(): number {\n    return this.items.length;\n  }\n}", ""),

    // 4. Enum
    ("enum Direction {\n  North,\n  East,\n  South,\n  West,\n}\n\nfunction opposite(d: Direction): Direction {\n  return (d + 2) % 4;\n}", ""),

    // 5. Discriminated union
    ("type Shape =\n  | { kind: \"circle\"; radius: number }\n  | { kind: \"square\"; side: number };\n\nfunction area(shape: Shape): number {\n  switch (shape.kind) {\n    case \"circle\":\n      return Math.PI * shape.radius ** 2;\n    case \"square\":\n      return shape.side ** 2;\n  }\n}", ""),

    // 6. Generic quicksort
    ("function quickSort<T>(arr: T[]): T[] {\n  if (arr.length <= 1) return arr;\n  const [pivot, ...rest] = arr;\n  const left = rest.filter((x) => x < pivot);\n  const right = rest.filter((x) => x >= pivot);\n  return [...quickSort(left), pivot, ...quickSort(right)];\n}", ""),

    // 7. Async with types
    ("interface Todo {\n  id: number;\n  title: string;\n  done: boolean;\n}\n\nasync function getTodo(id: number): Promise<Todo> {\n  const res = await fetch(`/todos/${id}`);\n  return (await res.json()) as Todo;\n}", ""),

    // 8. Utility types
    ("interface Config {\n  host: string;\n  port: number;\n}\n\nfunction connect(config: Readonly<Config>): void {\n  console.log(`Connecting to ${config.host}:${config.port}`);\n}\n\nconst settings: Partial<Config> = { port: 8080 };", ""),
];
