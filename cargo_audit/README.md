# cargo audit

https://github.com/RustSec/rustsec/tree/main/cargo-audit#rustsec-cargo-audit

## cargo auditとは

脆弱性を持つクレートの依存関係を[RustSec Advisory Database](https://github.com/RustSec/advisory-db/)から見つけるツール

## わかりやすい記事

https://zenn.dev/shinyay/articles/hello-rust-day072

## install

https://github.com/RustSec/rustsec/tree/main/cargo-audit#installation

```sh
$ cargo install cargo-audit --locked
```

## 実行

https://zenn.dev/shinyay/articles/hello-rust-day072#%E5%AE%9F%E8%A1%8C%E3%82%B5%E3%83%B3%E3%83%97%E3%83%AB

以下で実行する

```sh
$ cargo audit
```

何も問題が見つからなければ特に何も起こらない。

上記のリンクでは意図的に問題のあるクレートを入れて、どのように表示されるかを確認している。

## 脆弱性の修正

https://github.com/RustSec/rustsec/tree/main/cargo-audit#installation

脆弱性がある部分を修正する機能を追加するには、以下のinstallが必要

```sh
$ cargo install cargo-audit --locked --features=fix
```

これで以下を実行することで、問題のある依存関係が修正される

```sh
$ cargo audit fix
```

また以下で、実際に修正を行わない、お試し実行ができる

```sh
$ cargo audit fix --dry-run
```

### 修正ができない場合（未解決）

上記の「意図的に問題のあるクレートを入れる例」だと私の環境だと`cargo audit fix`で修正ができなかった

以下のようなエラーが出ていた

> This usually occurs when the fixed version is not semver-compatible,
> or the version range specified in your `Cargo.toml` is too restrictive

Cargo.tomlは以下のようになっている。

```toml
[dependencies]
conduit-hyper = "0.2.0"
```

## バイナリの検査

以下でバイナリの検査もできるようである。

```sh
$ cargo audit bin target/debug/test_audit
```

ただし、[`cargo auditable`](https://github.com/rust-secure-code/cargo-auditable)でコンパイルされていないと、一部の情報からしか判断ができないようである。

## 一部の脆弱性を無視、tomlファイルでの設定

https://github.com/RustSec/rustsec/tree/main/cargo-audit#ignoring-advisories

一部の脆弱性を無視したり、どれを無視するかを`audit.toml`で設定などができるようである

## CIでの使用

https://github.com/RustSec/rustsec/tree/main/cargo-audit#using-cargo-audit-on-travis-ci

https://github.com/RustSec/rustsec/tree/main/cargo-audit#using-cargo-audit-on-github-action

CIでの使用は上記を参照

github actionsの場合は[audit-check](https://github.com/rustsec/audit-check)があるようである。
