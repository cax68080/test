fn main(){
    //値を2倍にするクロージャーを定義する
    let x2 = |n| n*2;
    //x2を使ってみる
    println!("{}",x2(2));
    println!("{}",x2(8));
}