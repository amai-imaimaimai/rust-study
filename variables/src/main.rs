fn main() {
    let x = 5; // let mut x = 5; <- mutable variable
    println!("The value of x is: {}", x);
    // x = 6; // mutableである場合はエラーにならない
    // println!("The value of x is: {}", x);
    println!("Multiplied value of x is {}", multiply(x));

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The result of spaces.len() is {}", spaces);

    // let mut dots = "..."; <- compile error mutキーワードは変数の型まで変えられる訳ではない
    // dots = dots.len();
}

// let y = 10; compile error
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constantsはグローバルな場所に置ける

/**
 * JSdoc書けるのね
 */
fn multiply(x: u32) -> u32 {
    return x * THREE_HOURS_IN_SECONDS;
}
