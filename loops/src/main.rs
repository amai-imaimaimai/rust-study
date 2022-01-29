fn main() {
    // infinite loop
    // loop {
    //   println!("again!");
    // }

    labeled_loops();
    return_from_loop();
    nicer_loop_for_countdown();
}

fn labeled_loops() {
    let mut count = 0;
    'counting_up: loop {
        // 名前付きloop
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("reamining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // loopに名前を付けると、最も内側（innermost）のloopの代わりに名前付きloopをbreakすることができる
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn nicer_loop_for_countdown() {
    // 1..4の記法は標準ライブラリRangeのもの。この場合は1から（4の直前である）3までを生成する
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
