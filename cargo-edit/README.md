# cargo-edit

https://github.com/killercup/cargo-edit

## memo

### cargo-editとは

Cargoを拡張し、コマンドラインからCargo.tomlを修正して、依存関係を追加、削除、アップグレードすることができるようにするツール

### わかりやすい記事

https://zenn.dev/shinyay/articles/hello-rust-day070

### install

https://github.com/killercup/cargo-edit?tab=readme-ov-file

以下で行う

```
$ cargo install cargo-edit
```

必要なpackagesがある可能性がある。  
その場合は上記URLを参照

### add,rm

https://github.com/killercup/cargo-edit?tab=readme-ov-file#cargo-add

https://github.com/killercup/cargo-edit?tab=readme-ov-file#cargo-rm

addはCargo.tomlに依存関係を追加し、rmは削除する

addとrmはすでにcargoに統合されたらしい

若干使い方に違いがあるようである（上記URL参照）

### upgrade

https://github.com/killercup/cargo-edit?tab=readme-ov-file#cargo-upgrade

Cargo.tomlの依存関係を最新のものに更新する

更新するversionを指定したり、一部を更新しないように指定することができる（詳細は上記URL参照）

なお、cargo updateは、Cargo.lockに記録されている依存関係を更新するものであり、cargo upgradeとは異なる

### set-version

https://github.com/killercup/cargo-edit?tab=readme-ov-file#cargo-set-version

Cargo.tomlのversionを設定する

### 実行例

https://zenn.dev/shinyay/articles/hello-rust-day070

前半は上記と同様の内容

- regexの追加

```sh
$ cargo add regex
```

```toml
[dependencies]
regex = "1.10.4"
```

- regexのversionを指定しての追加

```sh
$ cargo add regex@1.6.0
```

```toml
[dependencies]
regex = "1.6.0"
```

- regexのfeatureを指定しての追加

```sh
$ cargo add regex --features="unicode"
```

```toml
[dependencies]
regex = { version = "1.6.0", features = ["unicode"] }
```

- regexのversionを最新のものに更新

```sh
$ cargo upgrade
```

```toml
[dependencies]
regex = { version = "1.10.4", features = ["unicode"] }
```

- regexの依存関係を削除

```sh
$ cargo rm regex
```

- Cargo.tomlのversionを設定する

```sh
$ cargo set-version 1.0.0
```

```toml
[package]
...
version = "1.0.0"
...
```

- Cargo.tomlのmajor versionを更新する

```sh
$ cargo set-version --bump major
```

```toml
[package]
...
version = "2.0.0"
...
```

- Cargo.tomlのminor versionを更新する

```sh
$ cargo set-version --bump minor
```

```toml
[package]
...
version = "2.1.0"
...
```

- Cargo.tomlのpatch versionを更新する

```sh
$ cargo set-version --bump patch
```

```toml
[package]
...
version = "2.1.1"
...
```
