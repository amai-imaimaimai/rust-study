fn main() {
  // sizeの最大値・最小値はこのコードを実行するCPUのアーキテクチャ（bit数）に依存する
  let arch_min = std::isize::MIN;
  println!("this machine's managed max integer is: {}", arch_min);
  let arch_max_unsigned = std::usize::MAX;
  println!(
    "this machine's managed max unsigned integer is: {}",
    arch_max_unsigned
  );
  let arch_max_signed = std::isize::MAX;
  // wrapping_addはオーバーフロー分をラップ（最小値からの値に変換）する
  let overflowed_but_wrapped = arch_max_signed.wrapping_add(1);
  println!("wrapped: {overflowed_but_wrapped}");
  // let overflowed = arch_max_unsigned + 1; // thread 'main' panicked at 'attempt to add with overflow'
  // println!("overflowed: {overflowed}");

  // 桁区切りの記法
  let dec = 98_222;
  println!("dec is: {} in decimal", dec);
  // 16進数
  let hex = 0xff;
  println!("hex is: {} in decimal", hex);
  // 8進数
  let oct = 0o77;
  println!("oct is: {} in decimal", oct);
  // 2進数
  let bin = 0b1111_0000;
  println!("bin is: {} in decimal", bin);
  // utf-8
  let byte = b'%';
  println!("byte is: {} in decimal", byte);
  // タプル
  let tup = (100, 200, "300", 400.0, '5');

  // destructuring
  let (a,b,c,d,e) = tup;

  println!("The values of tup: {a},{b},{c},{d},{e}00");

  // インデックスによるタプルの要素へのアクセス
  println!("The value of tup with index 1: {}", tup.0);

  // 値のないタプルはユニットタイプ・中の値はユニット値と呼ばれる
  // let tup_without_any_values = ();

  // `()` doesn't implement `std::fmt::Display`
  // println!("A tuple without any values: {}",tup_without_any_values)

  // 固定長配列（可変長配列はvectorと呼ばれ区別される）
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  // 各要素の値が3で要素数が5の固定長配列
  let a2 = [3; 5];
  let first_of_a = a[0];
  let last_of_a2 = a2[a2.len() - 1];
  println!("first is {}, last is {}", first_of_a, last_of_a2);
}