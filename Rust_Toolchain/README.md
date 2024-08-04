# Rust Toolchain

https://github.com/dtolnay/rust-toolchain

## Rust Toolchainとは

rustup を使って簡単に Rust toolchain をインストールする GitHub Actions

## できること

https://github.com/dtolnay/rust-toolchain?tab=readme-ov-file#example-workflow

GitHub Actions で [Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) をインストールすることができる。

例えば以下のようにインストールし、CIでテストなどを行うことができる。

```yaml
name: test suite
on: [push, pull_request]

jobs:
  test:
    name: cargo test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
```

なお、GitHub Actionsの利用方法などについては、以下などを参照

https://qiita.com/Kotabrog/items/0a4617bafceb9a112413

## オプション

`dtolnay/rust-toolchain@stable`の`@stable`の部分でツールチェーンの選択を行う。

例えば`@nightly`とすればnightlyのツールチェーンをインストールでき、`@1.80.0`とすれば1.80.0のversionをインストールできる。

また、以下のように入力として明示的に書くことも可能で、その場合は以下のような指定もできるようである。  
ただし、その場合は`@master`とする必要がある。

```yaml
     # Installs the most recent stable toolchain as of the specified time
     # offset, which may be written in years, months, weeks, or days.
  - uses: dtolnay/rust-toolchain@master
    with:
      toolchain: stable 18 months ago
```

```yaml
     # Installs the stable toolchain which preceded the most recent one by
     # the specified number of minor versions.
  - uses: dtolnay/rust-toolchain@master
    with:
      toolchain: stable minus 8 releases
```

https://github.com/dtolnay/rust-toolchain?tab=readme-ov-file#toolchain-expressions

その他のオプションとしては、以下がある。

- targets: インストールする追加ターゲットのカンマ区切り文字列: `wasm32-unknown-unknown`
- components: インストールする追加コンポーネントのカンマ区切り文字列: `clippy, rustfmt`

例えば以下のように使用できる。

```yaml
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo test --all-features
      - run: cargo fmt --all --check
```

## outputs

Rust Toolchainの出力には以下の2つがある。

- cachekey: インストールされた rustc バージョンの短いハッシュ（キャッシュキーとして使用するようである）: `20220627a831`
- name: 選択されたツールチェーンのバージョン名: `1.80.0`, `stable`

例えば以下のようにしてこれらの値を取得することができる。

```
    steps:
      - uses: actions/checkout@v4
      - id: toolchain
        uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: echo "${{steps.toolchain.outputs.cachekey}} ${{steps.toolchain.outputs.name}}"
```

## 使用例

以下はmainブランチへのpushとプルリクエスト時のみ`cargo test`と`cargo fmt`が実施されるようにした例である。

```yaml
name: Rust CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings

jobs:
  test:
    name: cargo test and fmt
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: test_rust
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo test --all-features
      - run: cargo fmt --all --check
```

Rust Toolchain以外の部分に関しては、以下の記事などを参照。

https://qiita.com/Kotabrog/items/0a4617bafceb9a112413
