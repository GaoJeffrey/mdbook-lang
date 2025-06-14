# Typescript
TypeScript 是一种强类型、面向对象的编程语言，设计用于将改语言编译为 JavaScript。它是 JavaScript 的超集，这意味着任何有效的 JavaScript 代码也是有效的 TypeScript 代码。

TypeScript 为 JavaScript 增加了静态类型，这意味着变量必须声明为特定的类型。这有助于在编译时而不是运行时捕获错误。

```typescript
let person: { name: string; age: number; isStudent: boolean } = {

    name: "John",
    age: 30,
    isStudent: true,
}
console.log(person);
```