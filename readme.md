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

