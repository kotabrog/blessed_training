# cargo-license

https://github.com/onur/cargo-license#cargo-license

## cargo-licenseとは

dependenciesのライセンスを確認するcargoの拡張ツール

## install

https://github.com/onur/cargo-license?tab=readme-ov-file#installation-and-usage

```sh
$ cargo install cargo-license
```

## 使用方法

https://github.com/onur/cargo-license?tab=readme-ov-file#example

```sh
$ cargo license
```

## 使用例

Cargo.tomlを以下のようにする。

```toml
[package]
name = "test_license"
version = "0.1.0"
edition = "2021"
license = "MIT/Apache-2.0"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

このとき、`cargo license`で以下のように表示された

```sh
Apache-2.0 OR Apache-2.0 WITH LLVM-exception OR MIT (1): wasi
Apache-2.0 OR MIT (8): cfg-if, getrandom, libc, ppv-lite86, rand, rand_chacha, rand_core, test_license
```

## Licenseについて

https://qiita.com/fate_shelled/items/a928709d7610cee5aa66

こちらなどがわかりやすかった

また、自分の記事だと以下なども

https://qiita.com/Kotabrog/items/fb328b72ac94137897af#licence%E3%81%AB%E3%81%A4%E3%81%84%E3%81%A6

## どのLicenseにすればいいか

https://sinkuu.github.io/api-guidelines/necessities.html

この記事によるとRustと同等のライセンスを適用する場合は`MIT/Apache-2.0`がいいかもしれない

## licenseを書く場所

https://sinkuu.github.io/api-guidelines/necessities.html

以下に書く（上記URL参照）

```toml
[package]
name = "..."
version = "..."
authors = ["..."]
license = "MIT/Apache-2.0"
```
