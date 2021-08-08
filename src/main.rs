



//高階関数書いた偶数二乗合計
//isize = i32 int型で入れてint型で返す
//関数宣言時、戻り値には変数名を書かないが、矢印を用いて返り値の型を定義する
//返り値はreturnで定義しなくても、最後の値が返り値となる
fn square_sum(n: isize)->isize{
    //0からnまでの区間をとる。
    (0..n)
    //リストやイテレータを返してTrueとなるものだけを返す
    // n=5の時, [2,4]のみiに入る
    .filter(|i| i % 2 == 0)
    // 要素を絶対値に変換
    //0からと定義しているため意味はないが、-1が入った場合は1として返される
    //n=5の時は、[4,16]となる
    .map(|i| i * i)
    //イテレータの合計を返す。
    //n=5の場合上の流れから[4,16]となっているため、20の値が返される
    .sum()

}

//不変変数の再定義
fn rebind(n: isize)->isize{
    let sum = 0;
    for i in 0..n{
        //再束縛sumですることができるが、このfor内でのforになる
        //代入ではなく、ここだけ使える新しい束縛したsumになる。
        //ここで以下を定義したとしても、ここで束縛したsumはfor外では使えず、
        //warningの原因になる
        //let sum = sum+i;
        //ここではあえて、先頭に_を接続することでwarningを削除し、あえて使わないsumを定義している。
        let _sum = sum+i;
    }
    return sum;
}

//可変変数の代入
fn reassign(n: isize)->isize{
    let mut sum = 0;
    for i in 0..n{
        sum = sum + i;
    }
    return sum;
}

fn main() {
    println!("{}", square_sum(5));
    println!("{}", rebind(5));
    println!("{}", reassign(5))
}
