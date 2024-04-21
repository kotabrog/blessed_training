# rustup

- https://rust-lang.github.io/rustup/index.html

## memo

### Rustupとは

https://rust-lang.github.io/rustup/concepts/index.html

rustupはツールチェーンマルチプレクサ

つまりRustツールチェーンをインストールして管理するもの

ツールチェーンとはインストールされたRustコンパイラ、関連ツール、標準ライブラリ一式を指す

https://solid.kmckk.com/SOLID/doc/latest/solid_rust/ecosystem/rust-toolchain.html

### rust-src

https://rust-lang.github.io/rustup/concepts/components.html

Rust標準ライブラリーのソースコードのローカルコピー。

Rustの実装を見るときにいいかもしれない

以下も参考になる

https://rust-lang.github.io/rustup/faq.html#can-rustup-download-the-rust-source-code

### minimal profile

https://rust-lang.github.io/rustup/concepts/profiles.html

CIで最小構成でrustの準備をしたいときにいいかもしれない

### rust update

https://rust-lang.github.io/rustup/basics.html

rustをupdateする

```sh
$ rustup update
```

### rustup update

https://rust-lang.github.io/rustup/basics.html

rustupをupdateする

```sh
rustup self update
```

### toolchain file

https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file

toolchain.tomlにツールチェーンを設定できる

### Cross-compilation

https://rust-lang.github.io/rustup/cross-compilation.html

ツールチェインを最初にインストールする際、rustupはホスト・プラットフォーム、つまり現在実行しているアーキテクチャとオペレーティング・システムの標準ライブラリのみをインストールする

他のプラットフォームにコンパイルするために必要な手順がここに

### environment variables

https://rust-lang.github.io/rustup/environment-variables.html

rustup関連の環境変数について

### Rustup Configuration

https://rust-lang.github.io/rustup/configuration.html

Rustupのsetting fileは ~/.rustup に

### Network proxies

https://rust-lang.github.io/rustup/network-proxies.html

Rustupのnetwork proxies設定

### rustup show

https://rust-lang.github.io/rustup/examples.html

現在のディレクトリでどのツールチェーンが使われるかを表示する

```sh
$ rustup show
```

### rustup dev

https://rust-lang.github.io/rustup/dev-guide/

rustupへのコントリビュートを行う場合はこちらを参照

### github

https://github.com/rust-lang/rustup?tab=readme-ov-file
