# insta

https://insta.rs/

## instaとは

スナップショットテストを行うためのライブラリ

## スナップショットテストとは

https://lab.mo-t.com/blog/rust-snapshot-testing

こちらを参照

また、以下では、スナップショットテストのメリット・デメリットについて詳しく書かれている

https://www.mizdra.net/entry/2021/02/04/003728

## install方法

https://insta.rs/docs/quickstart/

プロジェクト内で、以下の方法でdependenciesに追加することが推奨されいる。

```sh
cargo add --dev insta --features yaml
```

Cargo.tomlに直接記入する場合は、かわりに以下のようにする。

```toml
[dev-dependencies]
insta = { version = "1.39.0", features = ["yaml"] }
```

※ yaml以外の出力形式にしたい場合は、yaml以外を指定するか、何も指定しなくてもよい。どういったものが指定できるかは後述

また、必須ではないが、`cargo-insta`をインストールすることで機能が拡張されるようである。  
（cargo-instaの詳細はこちら：https://insta.rs/docs/cli/ ）

Unixの場合：

```sh
curl -LsSf https://insta.rs/install.sh | sh
```

cargoを使ったinstall

```sh
cargo install cargo-insta
```

## 最適化オプション

https://insta.rs/docs/quickstart/#optional-faster-runs

テスト用のクレートであるため、通常dev-dependenciesにのみ追加するが、devの最適化レベルはデフォルトでは0（全く最適化されない）である：https://doc.rust-lang.org/cargo/reference/profiles.html


そのため、コンパイルは若干遅くなるが、以下の設定を行うことで、使用するメモリが少なくなり、また動作も早くなるようである。

```toml
[profile.dev.package]
insta.opt-level = 3
similar.opt-level = 3
```

## 使用例

https://insta.rs/docs/quickstart/#reviewing-snapshots

サンプルとして以下を使用する。（main.rs）

```rust
fn split_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_split_words() {
    let words = split_words("hello from the other side");
    insta::assert_yaml_snapshot!(words);
}
```

次に以下を実行する

```sh
$ cargo insta test
```

すると`src/snapshots/{module name}__split_words.snap.new`が作成される。

次に以下を実行する

```sh
$ cargo insta review
```

すると、レビュー画面になる。

テストの結果を受け入れる場合は`a`を  
受け入れない場合は`r`を押す。

テストの結果を受け入れるとファイル名が`src/snapshots/{module name}__split_words.snap`になる。

その後、以下のように修正する。（sideを削除）

```rust
...
    let words = split_words("hello from the other");
...
```

そして以下を実行する

```sh
$ cargo test
```

すると、`assert_yaml_snapshot`される内容が異なるため、テストが失敗し、`src/snapshots/{module name}__split_words.snap.new`が作成される

次に以下を実行する

```sh
$ cargo insta review
```

すると、結果のdiffを確認することができる。

改めてテストの結果を受け入れる場合は`a`を  
受け入れない場合は`r`を押す。

上記の流れでテスト・レビューを行うことができる。

## 出力の比較方法

テストの結果はそのテストがあるディレクトリの`snapshots/{module name}__{test_name}.snap`（以下`.snap`と書く）に保存される。

この`.snap`の結果と比較して、異なる部分があると、テストが失敗するようになる。

異なる部分がある場合（新規にテストを実行する場合も含む）、テストがあるディレクトリに`snapshots/{module name}__{test_name}.snap.new`（以下`.snap.new`と書く）が作成される。

`cargo insta review`を使用すると、`.snap.new`を新しい結果として受け入れるか、それとも失敗として拒否するかを選択できる。

新しい結果として受け入れる場合は、`.snap.new`ファイルが新たな`.snap`ファイルとなる。

一方拒否した場合は`.snap.new`ファイルは削除される。

## cargo insta test

`cargo insta test`は、とりあえずすべてのテストを実行する。

例えば一つのテスト内に複数の`assert_yaml_snapshot`などがある場合、最初のassertで失敗すると、その後の出力を得られない。

そのため、とりあえずすべて実行して、結果を残したい場合は、`cargo insta test`を行うのがよい。

## cargo insta review

`cargo insta review`を実行するとレビューを開始できる。

`.snap.new`ファイルの内容を確認し、その内容を受け入れるかどうかを判断する。

受け入れると`.snap.new`ファイルは`.snap`ファイルになり、それが新しいテストの基準になる。

レビューでは以下を選択できる。

- a: accept: 新しいスナップショットを保持する
- r: reject: 古いスナップショットを保持する
- s: skip: 一旦スキップ
- i: hide info: 拡張スナップショット情報表示の切り替え（一回押すとshow infoになる）
- d: hide diff: スナップショットの差分表示の切り替え（一回押すとshow diffになる）

## gitignoreについての注意

https://insta.rs/docs/cli/#commands

デフォルトでは、.gitignoreや.ignoreがあると、instaはそれを参照してテストを無視する。

それ自体は問題ないが、無視される対象にsnapshotsディレクトリが入っていると、レビューができなくなってしまう。

そのため、snapshotsディレクトリを.gitignoreに加える場合は`--include-ignored`オプションをつけて以下のように、実行する必要がある。

```sh
$ cargo insta review --include-ignored
```

また、代わりにinstaの設定ファイルを設定する方法もある：https://insta.rs/docs/settings/#tool-config-file

## CIにおける注意

instaをCIで使用する際には、CI用の挙動にするために、`CI`環境変数が設定されていることを確認する必要がある

設定されていなければ以下のようにする

```sh
export CI=true
```

これを行うことで、例えば`.snap.new`ファイルが作成されなくなる

## シリアライズ形式

https://insta.rs/docs/serializers/

シリアライズ形式（YAML形式かJSON形式かなど）は使用するマクロを変更することで変えることができる。

以下マクロと形式のリストである。（どのような見た目になるかは上記URLを参照）

- `insta::assert_debug_snapshot!(&user_list);`: serdeクレートを使わずに、デフォルトのstd::fmt::Debug 表現を使用するinsta固有のシリアライザ
- `insta::assert_yaml_snapshot!(&user_list);`: YAML形式。推奨される形式
- `insta::assert_json_snapshot!(&user_list);`: JSON形式。推奨はされていない
- `insta::assert_compact_json_snapshot!(&range);`: コンパクトなJSON形式
- `insta::assert_toml_snapshot!(&user_list_in_struct);`: TOML形式。toml featureを有効にする必要がある。一部の表現が使えないようである。（詳細は上記URL参照）
- `insta::assert_ron_snapshot!(&user_list);`: RON形式。ron featureを有効にする必要がある。型名を含めることができる
- `insta::assert_csv_snapshot!(&user);`: CSV形式。csv featureを有効にする必要がある。一部のタイプの値に対してのみ機能する

また、`assert_snapshot!`は他とは違い、文字列のみサポートしている。

## Snapshot macro formats

https://insta.rs/docs/snapshot-types/#snapshot-types-list

snapshots macroが受け付ける引数の形式は以下になる。

それぞれについての詳細は後述

| Snapshot type | Example |
| ---- | ---- |
| File, named | assert_snapshot!("name", expr) |
| File, named, debug expr | assert_snapshot!("name", expr, "description") |
| File, unnamed | assert_snapshot!(expr) |
| File, redacted, named | assert_yaml_snapshot!("name", expr, {"." => sorted_redaction()}) |
| File, redacted, named, debug expr | assert_yaml_snapshot!(expr, {"." => sorted_redaction()}, "description") |
| File, redacted, unnamed | assert_yaml_snapshot!(expr, {"." => sorted_redaction()}) |
| Inline | assert_snapshot!(expr, @"result") |
| Inline, redacted | assert_snapshot!(expr, {"." => sorted_redaction()}, @"result") |

### Snapshot types

https://insta.rs/docs/snapshot-types/

スナップショットには大きく分けて以下の2つがある

- ファイルスナップショット：`.snap`ファイルに保存
- インラインスナップショット：コード内に保存

今までの内容（特に`.snap`ファイルや`.snap.new`ファイルに関するもの）はファイルスナップショットのものである。

インラインスナップショットも保存先が変わるだけで、そのほかは基本同じである。

### File, unnamed

https://insta.rs/docs/snapshot-types/#unnamed-snapshots

基本的には「出力の比較方法」に書いた形式でsnapshotsディレクトリに`.snap`ファイルと`.snap.new`ファイルが保存される。

このとき、`assert_snapshot!("first value");`のように、値だけの場合、ファイル名は`{module name}__{function name}.snap`になる。

ただし、例えば`test_something()`のように`test`がつくことが多いので、`test`は省略されて`{module name}__something.snap`になる。

また、例えば

```rust
#[test]
fn test_something() {
    assert_snapshot!("first value");
    assert_snapshot!("second value");
}
```

のように複数のassertがある場合は、それぞれ`something`、`something-2`となる。

### File, named

テストの名前は自分で決めることもできる。

`assert_snapshot!("name", "value")`とすれば、テストの名前は関数名ではなく`name`になる。

### Inline

インラインスナップショットを使用すると、コード内に結果を保存できる。

この機能は`cargo-insta`をインストールしないといけないようである。

インラインスナップショットを使用する場合は以下のようにする。

```rust
#[test]
fn test_inline() {
    insta::assert_yaml_snapshot!(vec![1, 2, 3], @"");
}
```

2つめの引数の`@""`の部分の@に続く文字列の部分がテストの結果が保存される場所になる。

例えばこの状態で`cargo insta test`を実行すると、`.{file name}.pending-snap`というファイルが作成される。

次に`cargo insta review`を実行すると、このファイルを元に、レビューが実施され、acceptを選択すると、`test_inline()`が書き換えられ、以下のようになる。

```rust
#[test]
fn test_inline() {
    insta::assert_yaml_snapshot!(vec![1, 2, 3], @r###"
    ---
    - 1
    - 2
    - 3
    "###);
}
```

### debug expr

第3引数に説明文を入れることができる。

例えば、以下の場合、`description`が説明文になる

```rust
insta::assert_yaml_snapshot!("snapshot", "value", "description");
```

## Redactions

https://insta.rs/docs/redactions/

この機能は`redactions`featureを有効にする必要がある。

「Snapshot macro formats」のredactedにあたる機能である。

これは、例えば構造体の中身の一部が乱数で決められるような場合に、その値を別の値で置き換える場合に使用する。

### 例

例えば以下のように使用する。

例

```rust
#[test]
fn test_redaction() {
    let vec = vec![1, 2, 3];
    insta::assert_yaml_snapshot!(vec, {
        "[2]" => 10,
    });
}
```

このようにすると、`vec`の3番目の要素は`10`として保存される。

### 構文

Redactionsは以下のように指定する。

```rust
insta::assert_yaml_snapshot!(..., {
    "selector" => replacement_value
});
```

selectorの一覧は以下を参照

https://insta.rs/docs/redactions/#selectors

また、`replacement_value`の代わりに、以下などを使用することもできる

- `dynamic_redaction`
    - コールバックを使用することができる
    - 詳細：https://insta.rs/docs/redactions/#dynamic-redactions
- `sorted_redaction`
    - 値をソートすることができる（`HashSet`のように順序が決まっていないものは、ソートしないと、順序がかわってスナップショットが異なってしまうかもしれない）
    - 詳細：https://insta.rs/docs/redactions/#sorted-redactions
- `rounded_redaction`
    - 少数の四捨五入ができる
    - 詳細：https://insta.rs/docs/redactions/#rounded-redactions

## Filter

https://insta.rs/docs/filters/

Redactionsの代わりにフィルター機能を使用することもできる。

これには`filters`featureが必要。

詳細は上記URLを参照

## Snapshot File

https://insta.rs/docs/snapshot-files/

スナップショットファイルの形式の詳細については上記URLを参照。

## その他の詳細機能

https://insta.rs/docs/advanced/

## 設定

https://insta.rs/docs/settings/

instaの設定については上記URLを参照
