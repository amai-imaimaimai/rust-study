fn main() {
    // Calling a function is an expression.
    another_function(5);
    stats_and_exps();
    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    // Calling a macro is an expression.
    println!("The value of x is: {}", x);
}

fn stats_and_exps() {
    // statements（文）でありexpressions（式）。関数の定義もstatements。statementsは値を返さない
    // Expressions can be part of statements
    // let y = 6;

    // expected expression, found statement (`let`)
    // let x = (let y = 6);
    // 他の言語でできるようなこういったこともできない↓
    // x = y = 6;

    // An expression. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement.
    let y = {
        let x = 3;
        x + 1 // セミコロンを足すと波括弧内は()を返す式に変わる
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // ここでもセミコロンを足すと()を返すようになる
}
