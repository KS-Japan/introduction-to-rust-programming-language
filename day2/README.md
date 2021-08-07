# day2
### 実行速度比較
#### RustとPythonの比較を行う
計測内容<br>
10,000,000までの素数の表示速度を計測する<br>
|  項目  |  sec  |
| ---- | ---- |
|  rust  |  50.242  |
|  python  |  95.633  |
|  python module Rust |  42.507  |

### Rustで作成したライブラリをpythonで呼ぶ方法
PyO3を利用するには、ツールチェインのnightly版をインストールする必要があります。
```
rustup install nightly 
rustup default nightly 
rustup toolchain list
```
Cargo.tomlにPyO3の依存関係を追記します。
```
[lib]
name = "eratosthenes"
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.13.2"
features = ["extension-module"]
```
ダイナミックライブラリを作成するために、`--release`をつけてビルドします。
```
cargo build --release
```
もし以下の様にエラーが出る場合は、バージョンを新しくすることで解決できる可能性があります。 <br>
参考：https://github.com/rust-osdev/x86_64/issues/234
```
#![cfg_attr(feature = "const_fn", feature(const_in_array_repeat_expressions))]
                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ feature has been removed
```
`cargo build --release`を実行すると、`target/release`内にWindowsであれば`.dll`, <br>
Linuxであれば`.so`、Macであれば`.dylib`が作成されます。<br>
作成されたファイルを以下対応拡張子に変更し、利用するpythonスクリプトと同じディレクトリに格納することでpythonで利用することができます <br>
|  OS  |  --releaseで作成されるファイル |   pythonで利用するために  |
| ---- | ---- | ---- |
|  Windows  |  .dll | .pyd |
|  Mac  |  .dylib  | .so |
|  Linux |  .so  | .so |

### 利用・参考ドキュメント
標準モジュール - time::Instant <br>
https://doc.rust-lang.org/std/time/struct.Instant.html <br>
PyO3 ライブラリ <br>
https://github.com/PyO3/pyo3 <br>
