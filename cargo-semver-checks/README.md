# cargo-semver-checks

https://github.com/obi1kenobi/cargo-semver-checks

## memo

### cargo-semver-checksとは

SemVerの仕様にのっとった互換性ルールをチェックするリンター

### わかりやすい記事

- https://zenn.dev/whiteshirt/articles/cargo-semver-checks

### SemVerとは

https://zenn.dev/whiteshirt/articles/cargo-semver-checks

Rust（cargo）ではSemantic Versioning（SemVer）を使用してバージョン管理を行っている。

https://ja.wikipedia.org/wiki/%E3%82%BB%E3%83%9E%E3%83%B3%E3%83%86%E3%82%A3%E3%83%83%E3%82%AF%E3%83%90%E3%83%BC%E3%82%B8%E3%83%A7%E3%83%8B%E3%83%B3%E3%82%B0

> 「1.23.45」といったように"."で区切った3つの数字で表される。
> 
> 前から順にメジャーバージョン、マイナーバージョン、パッチバージョンと呼ぶ。
> 
> APIの変更に互換性のない場合はメジャーバージョンを、後方互換性があり機能性を追加した場合はマイナーバージョンを、後方互換性を伴うバグ修正をした場合はパッチバージョンを上げる

https://semver.org/

SemVerの仕様

https://doc.rust-lang.org/cargo/reference/semver.html

また、こちらでライブラリの互換性を保つためのルールが文章化されている

### 使用方法

https://github.com/obi1kenobi/cargo-semver-checks?tab=readme-ov-file#quick-start

インストール方法などは上に

インストールされていればデフォルトの設定だと以下で確認できる

```sh
$ cargo semver-checks
```

また、github actionsを使用する際には以下でできる

```yaml
- name: Check semver
  uses: obi1kenobi/cargo-semver-checks-action@v2
```

### versionの参照

https://github.com/obi1kenobi/cargo-semver-checks?tab=readme-ov-file#does-the-crate-im-checking-have-to-be-published-on-cratesio

デフォルトではcrate.ioに公開してあるものを参照して互換性の確認を行う

ここにあるフラグを指定すれば、他の方法でも確認できる

### 何で比較しているか

https://github.com/obi1kenobi/cargo-semver-checks?tab=readme-ov-file#how-is-cargo-semver-checks-similar-to-and-different-from-other-tools

rustdocのjson出力からデータを取得して比較してる

### Contributing

https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md

コントリビュートしたい場合は上記を参照
