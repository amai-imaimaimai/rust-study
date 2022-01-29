fn main() {
    let condition = true;
    // ifは式（expression）なので、戻り値としてlet statementの右辺に使うことができる
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // `if` and `else` have incompatible types
    // let number = if condition {5} else {"six"};

    // `if` and `else` have incompatible types
    // if condition {
    // } else {
    //   5
    // };
}
