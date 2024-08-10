# Rust Cache

https://github.com/Swatinem/rust-cache

## Rust Cacheとは

Rustプロジェクト用のキャッシュを簡単に設定できるGitHub Action

## できること

Rustプロジェクトに適した形でのキャッシュを簡単に設定できる。

例えば以下のように利用することができる。

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
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features
```

なお、GitHub Actionsの利用方法などについては、以下などを参照

https://qiita.com/Kotabrog/items/0a4617bafceb9a112413

また`rust-toolchain`については以下を参照

https://zenn.dev/kotabrog/articles/e4506c6594efc7

## キャッシュ対象

https://github.com/Swatinem/rust-cache?tab=readme-ov-file#cache-details

主に以下のディレクトリをキャッシュする

- ~/.cargo: Cargoが使用するバイナリやレジストリ、キャッシュ、Git依存関係が含まれる
- ./target: ビルドされた依存関係の成果物が含まれている

これらを変更する方法については後述

また、基本的には依存関係のキャッシュを行う。  
ワークスペース内のクレート、つまりそのプロジェクト自体のコードは、頻繁に変更されるため、キャッシュしても効果が薄いという判断のようで、このアクションではキャッシュされないようである。

## 有効なプロジェクトについて

https://github.com/Swatinem/rust-cache?tab=readme-ov-file#cache-effectiveness

このアクションでは、プロジェクト自身のコードの成果物はキャッシュせず、そのプロジェクトが依存している外部クレートをキャッシュする。

そのため、依存関係が多いプロジェクトほど、キャッシュの恩恵を受けやすくなる。

またそのため、`Cargo.lock`がある方が効果が出やすい。  
もし`Cargo.lock`がない場合、毎回異なるバージョンの依存関係がビルドされる可能性があり、効果が薄くなる。

また、Rustコンパイラのバージョンについても同様で、Stableであればその更新は頻繁ではないが、Nightlyの場合は毎日更新されるため、キャッシュが無効化される可能性がある。  
そのため、Nightlyの場合は特定のバージョンに固定するなどしたほうがキャッシュが有効になるようである。

## キャッシュキーの生成

https://github.com/Swatinem/rust-cache?tab=readme-ov-file#cache-details

デフォルトではキャッシュキーは自動で生成されるが、その詳細については上記リンクを参照

## 不要なファイルの削除

https://github.com/Swatinem/rust-cache?tab=readme-ov-file#cache-details

不要なファイルをキャッシュしないように、以下が行われている。

- アクションが実行される前からあるファイルの削除
  - 例えば`rustc`など
- 使用されなくなった依存関係の削除
- 依存関係にないファイルの削除
  - プロジェクトで生成される一時ファイルやビルドに関係ないファイルなど
- インクリメンタルビルド関係のファイル
  - インクリメンタルビルドに必要となるファイルの生成やそのためのキャッシュなどを考えると、インクリメンタルビルドを活用しない方がよいという判断のようである
  - そのため、このアクションを使用する場合、CARGO_INCREMENTAL=0が設定され、インクリメンタル・コンパイルが無効にされるようである
- `mtime`（最後に修正された時間）をもとに1週間以上経過したビルド成果物を削除
  - 古いキャッシュが保存され続けることを防ぐ
- `~/.cargo/registry/src`ディレクトリはキャッシュされない。
  - このディレクトリは依存関係のソースコードが保存されているが、それより`~/.cargo/registry/cache`にある圧縮されたものを使用した方が高速だからである。

## オプション

基本的な使い方であれば、おそらく使用するとしても以下のオプションくらいである。

- workspaces
  - キャッシュするワークスペースとターゲットディレクトリの設定を指定する
  - `workspace -> target`という形式で指定する
    - デフォルトは`. -> target`

もし複数のworkspaceがある場合は以下のように指定する。

```yaml
- uses: Swatinem/rust-cache@v2
    with:
      workspaces: |
        workspace1 -> target1
        workspace2 -> target2
        workspace3 -> target3
```

なお、例えば以下のように`working-directory`を指定した状態でも、rust-cacheのために上記の方法で`workspaces`を指定する必要があることには注意

```yaml
  defaults:
    run:
      working-directory: test_rust
```

以下はその他のオプションについて

- `prefix-key`: キャッシュキーのプレフィックスを指定する
  - デフォルト: `v0-rust`
- `shared-key`: 自動生成されるjobベースのキーの代わりに使用する
  - デフォルト: 未設定
- `key`: 自動生成されるjobベースのキャッシュキーにさらに追加する部分の指定
  - デフォルト: 未設定
- `env-vars`: キャッシュキーに影響を与える環境変数のプレフィックスを指定
  - デフォルト: `CARGO CC CFLAGS CXX CMAKE RUST`
- `cache-directories`: ワークスペース外の追加のディレクトリをキャッシュに含める場合に指定（改行で区切る）
  - デフォルト: 未設定
- `cache-targets`: ワークスペースのtargetディレクトリをキャッシュするかどうかを指定する
  - デフォルト: `true`
- `cache-on-failure`: ワークフローが失敗した場合でもキャッシュを保存するかどうかを指定する
  - デフォルト: `false`
- `cache-all-crates`: すべてのクレートをキャッシュするかどうかを指定する
  - `true`に設定すると、依存関係にあるクレートだけでなく、プロジェクト内のすべてのクレートがキャッシュされる
  - デフォルト: `true`
- `save-if`: 特定の条件に基づいてキャッシュを保存したい場合に使用する
  - デフォルト: `true`
- `cache-provider`: キャッシュを提供するバックエンドを指定する
  - `github`か`buildjet`を指定できる
  - デフォルト: `github`

## 出力

出力は以下のみである。

- `cache-hit`: 指定されたキャッシュキーに一致するキャッシュが存在し、そのキャッシュが使用された場合に`true`を返す

例えば以下のように使用することができる。

```yaml
      - id: cache
        uses: Swatinem/rust-cache@v2
      - run: echo "${{steps.cache.outputs.cache-hit}}"
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
      - id: toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - id: cache
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: test_rust
      - run: cargo test --all-features
      - run: cargo fmt --all --check
```

rust-cache以外の点については前回の記事も参照

https://zenn.dev/kotabrog/articles/e4506c6594efc7#%E4%BD%BF%E7%94%A8%E4%BE%8B
