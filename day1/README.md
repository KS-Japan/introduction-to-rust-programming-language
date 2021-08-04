## day1
### ・Hello RUST
ソース：hello_rust.rs <br>
目的：簡易的な関数の使い方を学ぶ

### ・数当てゲーム
パッケージ名：guessing_game <br>
目的：変数やパッケージ読み込みなど基本的な使い方を学ぶ

#### 標準パッケージではない場合、クレートを利用して機能を追加する<br>
```
use rand::Rng;
```
何もしないままだと上記は利用できないが、以下を「Cargo.toml」の[dependencies]セクションヘッダに追加することで利用することができる。
```
[dependencies]
rand = "0.3.14"
```

