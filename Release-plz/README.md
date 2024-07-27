# Release-plz

https://release-plz.ieni.dev/

## Release-plzとは

RustプロジェクトのCIでリリース用のプルリクエストを自動作成してくれるツール

## できること

- [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) に基づくバージョン更新
- [git-cliff](https://git-cliff.org/) による変更履歴の更新。デフォルトでは [keep a changelog](https://keepachangelog.com/en/1.1.0/) フォーマットを使用
- [cargo-semver-checks](https://zenn.dev/kotabrog/articles/2c413ed59166ef) によるAPIの変更検出
- Cargoワークスペースに対応
- 設定なしで実行可能
- リリース前のカーゴアップデート（オプション）
- リリースされたパッケージごとに Git タグを作成
- cargoレジストリへのパッケージ公開
- GitHub/Giteaでのリリース

## 使い方

チュートリアルがわかりやすいため、以下を見るのがよい。

https://release-plz.ieni.dev/docs

## ローカルでの使用

詳細については以下を参照

https://release-plz.ieni.dev/docs/usage

ローカルでは以下でインストールできる

```rust
$ cargo install release-plz --locked
```

その他のインストール方法については以下

https://release-plz.ieni.dev/docs/usage/installation

また、以下が実行できる  
（それぞれの詳細は、チュートリアルを参照）

- `release-plz update`: 変更をコミットせずにローカルでプロジェクトを更新する
- `release-plz release-pr`: GitHub Pull Request を作成
- `release-plz release`: パッケージの新バージョンを公開
- `release-plz init`: 現在の GitHub リポジトリ用に release-plz を初期化
- `release-plz set-version`: Cargo.tomlとchangelogでパッケージのバージョンを編集
- `release-plz generate-completions`: シェルのコマンド補完を生成
- `release-plz generate-schema`: release-plz 設定用の JSON Schema を生成

## GitHub Actionsでの使用

詳細については以下を参照

https://release-plz.ieni.dev/docs/github

デフォルトでは、コミットをメインブランチにマージするたびに、以下を実行する

- `release-plz release-pr`: リリース Pull Request を作成
- `release-plz release`: 未公開パッケージを公開

以下の Quickstart を参考に設定するのがよさそう。  
（release-pr と release のどちらも行う例とそれぞれのみの例が載っている）

https://release-plz.ieni.dev/docs/github/quickstart

## Googleのrelease-pleaseとのちがいについて

https://release-plz.ieni.dev/docs/why#differences-with-release-please

似たプロジェクトとしてGoogleのrelease-pleaseがあるが、そちらとは違う点が挙げられている。  
（以下はRelease-plzの特徴）

- 設定ファイルが必要ない
- Gitタグがない場合、Cargo Registryを参照
- 専用のコミットメッセージを行う必要がない
- Rust言語のみのプロジェクトにしか対応していない
