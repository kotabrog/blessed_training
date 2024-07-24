# cargo-expand

https://github.com/dtolnay/cargo-expand

## cargo-expandとは

マクロと#[derive]を展開して確認できるcargoの拡張ツール

## install

https://github.com/dtolnay/cargo-expand?tab=readme-ov-file#installation

以下でインストールできる。

```sh
$ cargo install cargo-expand
```

[rustfmt](https://zenn.dev/kotabrog/articles/683a6956233034)を使用して展開した結果を整えるため、rustfmtが使えない状態だと、結果が見づらくなるようである。

## 使い方

https://github.com/dtolnay/cargo-expand?tab=readme-ov-file#example

以下でプロジェクト内のコードのマクロなどを展開して出力する。

```sh
$ cargo expand
```

上記リンクのコードと同じものであるが、例えば`src/main.rs`が以下のようだと、出力は以下のようになる。

```rust
#[derive(Debug)]
struct S;

fn main() {
    println!("{:?}", S);
}
```

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct S;
#[automatically_derived]
impl ::core::fmt::Debug for S {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "S")
    }
}
fn main() {
    {
        ::std::io::_print(format_args!("{0:?}\n", S));
    };
}
```

## オプション

`cargo expand --help`の結果は（現在だと）以下のようになっている。  
（helpの表示がわかりやすいので、詳細は省略する）

```sh
Usage: cargo expand [OPTIONS] [ITEM]

Arguments:
  [ITEM]  Local path to module or other named item to expand, e.g. os::unix::ffi

Options:
      --ugly                Do not attempt to run rustfmt
      --theme <NAME>        Select syntax highlighting theme
      --themes              Print available syntax highlighting theme names
      --verbose             Print command lines as they are executed
      --color <WHEN>        Syntax highlighting and colored Cargo output (auto, always, never)
      --config <KEY=VALUE>  Override a configuration value
  -Z <FLAG>                 Unstable (nightly-only) flags to Cargo
      --version             Print version
  -h, --help                Print help

Package Selection:
  -p, --package [<SPEC>]  Package to expand

Target Selection:
      --lib               Expand only this package's library
      --bin [<NAME>]      Expand only the specified binary
      --example [<NAME>]  Expand only the specified example
      --test [<NAME>]     Expand only the specified test target
      --tests             Include tests when expanding the lib or bin
      --bench [<NAME>]    Expand only the specified bench target

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs
      --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
      --target <TARGET>         Target triple which compiles will be for
      --target-dir <DIRECTORY>  Directory for all generated artifacts

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --frozen                Require Cargo.lock and cache are up to date
      --locked                Require Cargo.lock is up to date
      --offline               Run without accessing the network
```

## コンフィグ

https://github.com/dtolnay/cargo-expand?tab=readme-ov-file#configuration

`$CARGO_HOME/config.toml`（通常は`~/.cargo/config.toml`）内の`[expand]`に設定を書くことができる。

詳細は上記リンク参照。

## 非可逆性について

https://github.com/dtolnay/cargo-expand?tab=readme-ov-file#disclaimer

マクロの展開は非可逆であるようである。  
つまり、展開されたものはコンパイルができるとは限らず、また、同じ動作をするとは限らない。

例えば上記リンクにある以下のコードを考える。

```rust
fn f() -> i32 {
    let x = 1;

    macro_rules! first_x {
        () => { x }
    }

    let x = 2;

    x + first_x!()
}
```

これは`cargo expand`によって展開すると以下のようになる。

```rust
fn f() -> i32 {
    let x = 1;
    let x = 2;
    x + x
}
```

展開前の関数の出力は3であるが、展開後は4になってしまう。

そのため、あくまでデバッグ用として、マクロがどのように展開するかを確認する用途で使うのがいいようである。
