# clippy

- https://github.com/rust-lang/rust-clippy
- https://doc.rust-lang.org/nightly/clippy/index.html

## memo

### clippyとは

Rustコードを改善するためのリンター

### github

https://github.com/rust-lang/rust-clippy

### リスト

https://rust-lang.github.io/rust-clippy/master/index.html

### 使い方

https://github.com/rust-lang/rust-clippy?#usage

こちらを参照

installなどしていたら基本的に以下で実行

```sh
$ cargo clippy
```

### CIで

https://github.com/rust-lang/rust-clippy?tab=readme-ov-file#travis-ci

以下のようにすることでwarning時にもCIで失敗するようになる。（ワーニングでエラーになるようになる）

```sh
$ cargo clippy -- -D warnings
```

※ ただし、この場合、rustcが見つけたwarningなどもエラーになる。回避法はリンクに  
※ RUSTFLAGS: "-Dwarnings"とすることで上記のフラグをつけなくてもよくなる

また以下でテストやデフォルト以外のターゲットも確認するようになる

```sh
$ cargo clippy --all-targets --all-features -- -D warnings
```

### configuration

https://github.com/rust-lang/rust-clippy?tab=readme-ov-file#configuration

一部を無効にしたりなど色々な設定はこちら

### Contribute

https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md

コントリビュートを行う場合はこちらを参照

### clippy book

https://doc.rust-lang.org/nightly/clippy/index.html

より詳しいドキュメント

### RFC

https://github.com/rust-lang/rfcs/blob/master/text/2476-clippy-uno.md

