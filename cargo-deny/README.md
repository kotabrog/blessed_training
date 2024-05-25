# cargo-deny

https://github.com/EmbarkStudios/cargo-deny

## cargo-denyとは

プロジェクト内の様々な確認を行うcargoの拡張ツール

## わかりやすいサイト

- https://qiita.com/KisaragiEffective/items/30071957ae4174fdaf1a

## install

https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#install-cargo-deny

```sh
$ cargo install --locked cargo-deny
```

## 使用前の準備

https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#initialize-your-project


まずはプロジェクト内で設定ファイルのinitを行う

```sh
$ cargo deny init
```

このようにすると`deny.toml`が作成される

設定ファイルがあるのであれば、それをコピーしてきてもよい

## 使い方

https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#usage

以下のコマンドでプロジェクトの確認を行う

```sh
$ cargo deny init
```

確認項目は以下の4つで、それぞれのみについて確認することもできる

※ 詳細な設定についてまでは調べていない。こちらを参照：https://embarkstudios.github.io/cargo-deny/index.html

### licenses

- https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#licenses
- https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html

```sh
$ cargo deny check licenses
```

依存関係のlicenseが`deny.toml`で設定した条件を満たしているかどうかを確認する

> cargo-denyは、SPDX フォーマットを使用してクレートのライセンス要件を解釈します。  
> メタデータからSPDX フォーマットを直接取得できない場合は、指定した信頼閾値内で SPDX フォーマットを導出しようとします

SPDX license expressions

- https://spdx.github.io/spdx-spec/v3.0/annexes/SPDX-license-expressions/
- https://ja.wikipedia.org/wiki/Software_Package_Data_Exchange


### Bans

- https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#bans
- https://embarkstudios.github.io/cargo-deny/checks/bans/index.html

```sh
$ cargo deny check bans
```

特定のクレートを拒否、または許可したり、同じクレートの複数のバージョンについて検出することを行う

※ Rustでは、同じクレートに依存する別のクレートなどがあった場合に、それぞれが違うバージョンに依存していると、そのどちらもプロジェクトに含めるようなことがあるようである：https://embarkstudios.github.io/cargo-deny/checks/bans/index.html#use-case---duplicate-version-detection

### Advisories

- https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#advisories
- https://embarkstudios.github.io/cargo-deny/checks/advisories/index.html

```sh
$ cargo deny check advisories
```

アドバイザリ・データベースを調べて、クレートの問題を検出する

- セキュリティ脆弱性の検出
    - [advisory database](https://embarkstudios.github.io/cargo-deny/checks/advisories/index.html)をもとに検出
    - 自分のデータベースをもとに検出するようにすることも可能
- メンテナンスされていないクレートの検出
    - 同様にadvisory databaseから検出

### sources

- https://github.com/EmbarkStudios/cargo-deny?tab=readme-ov-file#sources
- https://embarkstudios.github.io/cargo-deny/checks/sources/index.html

```sh
$ cargo deny check sources
```

クレートが信頼できるソースからのみきているか確認する

また、gitやレジストリのソースなどをすべて無効にすることで、外部に全く依存しないような設定にして、それが守られているかを確認することも可能。

## GiuHub Actions

https://embarkstudios.github.io/cargo-deny/index.html#github-action

GitHub Actionsで使用したい場合は上記URLなどを参照
