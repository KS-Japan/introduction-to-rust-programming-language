
use std::io;//ioライブラリ
use std::cmp::Ordering;//標準ライブラリ
use rand::Rng;//乱数ライブラリ

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    //不変変数の初期化 単体では使えず宣言したものを使わないとwarning が発生する.
    //意図的な宣言であれば、_を先頭につけることで単体宣言ができる。
    //所有権の束縛：束縛された変数に以後アクセスできなくなる
    let mut guess = String::new();
    //入力したデータを読み取るにはioライブラリのstdin()は不要
    //可変借用型&mut　変数名ではなく参照が渡される
    //rustのデフォルトは不変（immutable）のためmutをつけて可変にしている
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");//例外（エラー）が発生した場合はメッセージを表示する
    //上記で入力した型は自動的にString型になるが、ランダム数字と比較するためにu32の数値型に変更する
    //新しい型でguessを覆い隠す
    //ユーザーが入力すると"[数字] \n"となる。.trim()を用いることで「改行」を外すことができる
    //parse()は、文字列を数値に変換するため、定義しているu32と一致しエラーが出ない
    //parse()は、記号が含まれている文字列も多くエラーになりやすい
    let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
    //入力した変数を表示
    println!("You guessed: {}", guess);
    //乱数生成
    //不変変数　1~10の間で生成される
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("The secret number is: {}", secret_number); 
    //比較 現状ではコンパイルできない部分
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"), 
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");  
        }
    }
}

