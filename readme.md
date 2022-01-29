# Rust Study

## Cargo

* クロスプラットフォームで同じコマンドを使える
### cargo check

* コンパイルが通ること（エラーがないか）を確認する
* コードの構造が同じ場合は、確認をスキップする（`Finished`のみ表示される）

## Conventions of Writing Codes
### trade-offs of immutable variables

> There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

### constants

> Rust’s naming convention for constants is to use all uppercase with underscores between words. The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify

## types

### integer

> The primary situation in which you’d use isize or usize is when indexing some sort of collection.

### floating-point

> The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

### tuple

> The term originated as an abstraction of the sequence: single, couple/double, triple, quadruple, quintuple, sextuple, septuple, octuple, ..., n‑tuple, ..., where the prefixes are taken from the Latin names of the numerals.

https://en.wikipedia.org/wiki/Tuple

> In Haskell, Rust, and Elm, the unit type is called () and its only value is also (), reflecting the 0-tuple interpretation.

> In JavaScript, both Null (its only value is null) and Undefined (its only value is undefined) are built-in unit types.

https://en.wikipedia.org/wiki/Unit_type

### array

> Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.

## functions

### statements and expressions

> Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. 