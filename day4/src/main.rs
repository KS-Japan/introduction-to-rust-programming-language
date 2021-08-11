//所有権
//メモリはガベージコレクションやメモリの確保開放で管理をするのが一般出来だが、Rustは第3の手法をとっている
//メモリは、コンパイラがコンパイル時にチェックする一定の規則とともに所有権システムを通じて管理されている
//どの所有権機能も動作を遅くなることはない

//スタック
//last in, first out:祭syとに入れたものを最後に出す
//データを追加することをpush, データを取り除くことをpopという
//スタックは高速
//スタック上のデータは全て既知の固定サイズでなければならない

//ヒープ
//ヒープにデータを置く際に、空の領域を見つけ、領域を使用中にし、ポインタを返す
//ポインタとは、その場所へのアドレス
//この過程は、ヒープに領域を確保する(allocating on the heap)と呼ばれ、時としてそのフレーズを単にallocateするなどと省略したりします。
//スタックのデータへのアクセスよりも低速
//ヒープに大きな領域を確保する行為も時間がかかることがある

fn scope(){
    //このスコープ("{}")で有効になる。このスコープ抜けるまで有効なまま
    let _s = "Hello";
}

fn string(){
    //from関数を利用して、文字列リテラルからString型を生成する
    //2種類目の文字列型、String型がある。この型はヒープにメモリを確保するため、
    //コンパイル時にはサイズが不明なテキストも保持することができる
    let mut _s = String::from("Hello");
    _s.push_str(", World!");
    println!("{}",_s);
}

fn _move(){
    //値5をxに束縛する、xの値をコピーしてyに束縛する。
    //整数は既知の固定サイズの単純な値で、二つの５の値はスタックに積まれる
    let x = 5;
    let _y = x;
    //上の処理を同じように見えるが異なっている
    //Stringは、ptr(文字列の中身を保持するメモリへのポインタ),len(長さ),capacity(許容量 byte)の
    //3つの部品でできており、さらにptrはindexとvalueとなっている。
    let s1 = String::from("Hello");
    //ポインタ情報をコピーした際、ポインタと長さ、許容量はコピー
    //ただし、ポインタが指すヒープ上のデータはコピーされず同じ場所を参照することになる
    let s2 = s1;
    //println!("{}, rust!", _s2);
    //a1とs2の両方が同じデータポインタを指していることは問題になる。
    //rustはスコープから外れると自動でdrop関数を呼び出し、メモリ開放を行う。
    //二重開放は、memory corruption(メモリ崩壊)の原因になり、セキュリティ上の脆弱性を生むことになる
    //println!("{}, rust!", s1); //エラーになる

    //rustはメモリをコピーする代わりに、コンパイラはa1は有効ではないと考えs1の参照を無効にする。
    //ポインタと長さ、許容量をコピーする考えは、ほかの言語の"shallow copy"に似ているが、
    //rustは変数ごと無効にするため、区別するためにムーブと名付けている
    println!("{}, rust!", s2);
}

fn _clone(){
    //異なる変数が同じポインタを指していることが問題だったが、
    //同じヒープデータを別のメモリに格納することで、二つの変数とも利用することができる
    //その分実行コストが高い
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
}

fn not_move(){
    //ヒープの場合はコピーされるとムーブが実行され、
    //コピー元が利用停止できなくなるが、
    //整数のような既知のサイズを持つ型はスタックは例外扱いされcopyされる
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返す
    (s, length)
}

fn ex1(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn main() {
    scope();
    string();
    _move();
    _clone();
    not_move();
    ex1();
}
