# rustfmt

https://github.com/rust-lang/rustfmt

## memo

### rustfmtとは

Rustコードをスタイルガイドラインに従ってフォーマットするツール

### github

https://github.com/rust-lang/rustfmt

### contributing

https://github.com/rust-lang/rustfmt/blob/master/Contributing.md

コントリビュートしたいときはこちらを参照

### 使い方

https://github.com/rust-lang/rustfmt?tab=readme-ov-file#quick-start

インストールしていれば以下で実行

```sh
$ cargo fmt
```

インストール方法などは上記のリンクより

### 修正を行わないモード

デフォルトでは`cargo fmt`で修正まで行うが、`--check`をつけることで、修正を行わず、修正内容を教えてくれるようになる。

また、このときエラー扱いになるため、CIなどで使用する場合は`--check`をつけるのがいいかもしれない

### all

`--all`をつけることで以下のようになる。

> Format all packages, and also their local path-based dependencies

しかし、詳しい範囲までは調べていない。

### Config

https://rust-lang.github.io/rustfmt/?version=master&search=

rustfmt.tomlで設定できる

内容については上記を参照

### Rust Style Guide

https://doc.rust-lang.org/nightly/style-guide/

Rustスタイルガイドでは、デフォルトのRustスタイルを定義している。

### rustfmtを無視したい場合

rustfmtを無視したい場合は`#[rustfmt::skip]`をつける

マクロや属性のrustfmtを防ぐためには、`#[rustfmt::skip::macros(target_macro_name)]`や`#[rustfmt::skip::attributes(target_attribute_name)]`を使用する

### 参考になる記事

- https://zenn.dev/shinyay/articles/hello-rust-day069

