# cargo release

https://github.com/crate-ci/cargo-release

## cargo releaseとは

Rustプロジェクトのリリース作業を自動化するクレート

## 機能

https://github.com/crate-ci/cargo-release?tab=readme-ov-file#cargo-release

以下を行うことができる。

- リリースに適した状態であることを確認する
  - ブランチが正しいか
  - 最新の状態か
  - clean treeな状態か（コミットされていない変更などがないか）
- ワークスペースのサポート
- cargo publishの実行、タグ付け、プッシュを自動化
- 変更ログの更新やDockerfileのタグ更新

## install

https://github.com/crate-ci/cargo-release?tab=readme-ov-file#install

```sh
$ cargo install cargo-release
```

## 前提条件

https://github.com/crate-ci/cargo-release?tab=readme-ov-file#prerequisite

プロジェクトは git で管理されている必要がある。

## 使用方法

https://github.com/crate-ci/cargo-release?tab=readme-ov-file#usage

リリースを行う際は以下のコマンドを使用する。

```sh
$ cargo release [level]
```

`level`はリリースのレベル（major, minor, patchなど）を指定することができる。

LEVELやそのほかのオプションなどの詳細は以下を参照

https://github.com/crate-ci/cargo-release/blob/master/docs/reference.md#cli-arguments

## Dry run

https://github.com/crate-ci/cargo-release?tab=readme-ov-file#dry-run

デフォルトではdry-run実行になっている。  
つまり、変更は実際には実施されず、何が実行されるかを確認することができる。

実際に実行したい場合は`--execute`をつける。

## Config

Configについては以下を参照

https://github.com/crate-ci/cargo-release/blob/master/docs/reference.md#configuration
