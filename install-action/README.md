# install-action

https://github.com/taiki-e/install-action

## install-actionとは

Rustのための開発ツールをインストールするためのGitHub Action

## できること

https://techblog.paild.co.jp/entry/2023/04/10/170218

GitHub Actionsで使用したいRustのツールをインストールすることができる。

特に以下の2点でメリットがある。

- 基本的にバイナリを取ってきてくれるので、ビルドを行わない分CIの時間を短縮できる
- セキュリティ面

（詳細は後述）

例えば以下のように使用できる。

```yaml
name: Rust CI
on: [push, pull_request]

jobs:
  test:
    name: cargo test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@cargo-nextest
      - run: cargo nextest run --all-features
```

上記の`uses: taiki-e/install-action@cargo-nextest`で`cargo-nextest`をインストールしている。

## 使用方法

https://github.com/taiki-e/install-action?tab=readme-ov-file#usage

インプットは以下の3つがある。

- tool: インストールするツール名を「,」区切りで指定
- checksum: チェックサムを行う場合true
  - デフォルトはtrue
- fallback: フォールバック（サポートしていないツールの場合にとる方法）をどうするか
  - デフォルトは`cargo-binstall`。`cargo-binstall`を使用して指定したツールをインストールする
  - noneを指定するとフォールバックを行わない

### tool: versionの指定方法や複数指定について

https://github.com/taiki-e/install-action?tab=readme-ov-file#example-workflow

以下のようにversionを指定することができる。  

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-hack@0.5.24
```

なお、versionの指定方法は以下がある。

- すべて指定：1.5.24
- パッチバージョンの省略：1.5
- メジャーバージョンが1以上の場合はマイナーバージョンの省略：1
- 指定しない

なお、versionを指定しない場合は、以下のような省略記法が使える。

```yaml
- uses: taiki-e/install-action@cargo-hack
```

また、複数指定する場合は以下のように指定する。

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-hack,cargo-minimal-versions
```

### checksum: セキュリティについて

GitHub Releasesからインストールするツールについては、ダウンロードしたファイルの[SHA256チェックサム](https://atmarkit.itmedia.co.jp/ait/articles/0507/30/news017.html)を検証する。

これはデフォルトで有効になっており、以下のようにchecksumをfalseにすることで無効にできる。

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-hack@0.5.24
    checksum: false
```

ツールが署名付きアーカイブを配布している場合は、署名の検証も行う。  
署名検証はチェックサムを取得する段階で行われるため、チェックサムを無効にすると署名検証も無効になる。

また、checksumとは関係がないが、GitHub Releasesからツールをインストールする場合、tlsv1.2+のHTTPSを使用してツールまたはそのインストーラーをダウンロードする。  
これはrustupの推奨インストールと同じレベルのセキュリティのようである。

### fallback: サポートしているツールとそうでない場合

https://github.com/taiki-e/install-action?tab=readme-ov-file#supported-tools

https://techblog.paild.co.jp/entry/2023/04/10/170218

このアクションでは、以下のリンクのリストにあるものを対応している。

https://github.com/taiki-e/install-action/blob/main/TOOLS.md

このリストにないものの場合は、デフォルトだと[cargo-binstall](https://github.com/cargo-bins/cargo-binstall)を使用してインストールを試みる。

もしこれを行わないようにしたい場合は以下のように`fallback`を`none`にする必要がある。

```yaml
- uses: taiki-e/install-action@v2
  with:
    tool: cargo-hack@0.5.24
    fallback: none
```

なお、例えば`cargo install ...`とインストールする場合は、ソースをインストールして、そのあとビルドを行う。

CIで使用する場合は、このビルド時間がもったいないので、バイナリをそのままインストールできると便利だが、それを行うのが`cargo-binstall`である。

一方`cargo-binstall`の場合はcrate metadataからパースして最適なインストール処理を見つけて実行するという設計になっているようで、そのためインストール手順が常に同じとは限らない。

「サポートしているツール」については`cargo-binstall`を使用しせずに特定のインストール方法でインストールしているため、この問題が起きないようになっていると思われる。


## 使用例

以下はmainブランチへのpushとプルリクエスト時のみ`cargo nextest`と`cargo fmt`が実施されるようにした例である。

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
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-nextest
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: test_rust
      - run: cargo nextest run --all-features
      - run: cargo fmt --all --check
```

install-action以外の点については前回の記事も参照

https://zenn.dev/kotabrog/articles/a3c3033f96e9ec

また`cargo-nextest`については以前の記事を参照

https://zenn.dev/kotabrog/articles/974468ac37639d
