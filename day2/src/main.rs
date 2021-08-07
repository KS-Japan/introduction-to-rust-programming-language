use std::time::Instant;//時間計測ライブラリ
//PyO3を利用することにより、pythonで利用することができる
//use pyo3::prelude::*;
//use pyo3::types::PyList;

fn main() {
    let start = Instant::now();//.計測開

    for i in 1..10000000{
        if is_prime(i){
            println!("{}",i);
        }
    }
    let end = start.elapsed();//計測終了
    //sybsec_nanosでnano sec単位で取得します。
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);

}

//素数判定関数
fn is_prime(n:u32)->bool{
    if n < 2{
        false
    } else if n==2{
        true
    } else if n%2==0{
        false
    } else{
        let mut i =3;
        while i*i<=n{
            if n%i ==0{
                return false;
            }
            i += 2;
        }
        true
    }
}