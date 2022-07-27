//Rustで素数を100個生成する

//素数判定する関数
fn is_prime(n:usize) -> bool{
    for i in 2 .. n{
        if n % i == 0{
            return false
        }
    }
    return true
}

//素数を100個求める関数
fn get_primes(primes:&mut[usize;100]){
    let mut i = 2;
    let mut count = 0;
    //countが100になるまでくりかえす
    while count < 100{
        if is_prime(i){
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main(){
    //初期値０の配列を１００個用意する
    let mut primes = [0;100];
    //素数を１００個求める
    get_primes(&mut primes);
    //結果を表示する
    println!("{:?}",primes)
}