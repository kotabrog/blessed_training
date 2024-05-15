# cargo-outdated

https://github.com/kbknapp/cargo-outdated#cargo-outdated

## cargo-outdatedとは

古くなった依存関係を表示するcargoの拡張機能

## わかりやすい記事

https://zenn.dev/shinyay/articles/hello-rust-day071

## install

https://github.com/kbknapp/cargo-outdated?tab=readme-ov-file#installing

```sh
$ cargo install --locked cargo-outdated
```

※ `--locked`を使用すると`Cargo.lock`を使用するようになる

## Caret requirementについて

https://qiita.com/tesaguri/items/f06409c13df428e1152d

> Caret requirementは、クレートのあるバージョンに対してSemVer的に互換（SemVer compatible）なバージョンの範囲を指定します。具体的には、X.Y.Zの３つの番号のうち、最も左の0でない番号（ただし全て0のときは最も右側の0）を保つようなアップデートのみを許容します。

> Caret requirementはCargo.comlのdependenciesのバージョン指定におけるデフォルトの表記です。

つまり、例えばCargo.tomlに`clap = "2.20.0"`と書いたとしても、実際に使用されるのは`>=2.20.0 and < 3.0.0`となるversionである。

（例えば以下の例だとrustfftについてはそうではなく、例外もあるようである。crateを公開する際にそういった設定をできるのかもしれないが、詳細についてはわかっていない）

## 例と各列について

例えばCargo.comlを以下のようする

```toml
[dependencies]
clap = "2.20.0"
rustfft = "1.0.0"

[dev-dependencies]
rand = "0.7.0"
```

このとき、現在の私の環境だと以下のようになる

```sh
Name                                  Project                       Compat  Latest                         Kind         Platform
----                                  -------                       ------  ------                         ----         --------
ansi_term->winapi                     0.3.9                         ---     Removed                        Normal       cfg(target_os = "windows")
atty->hermit-abi                      0.1.19                        ---     Removed                        Normal       cfg(target_os = "hermit")
atty->libc                            0.2.154                       ---     Removed                        Normal       cfg(unix)
atty->winapi                          0.3.9                         ---     Removed                        Normal       cfg(windows)
clap                                  2.34.0                        ---     4.5.4                          Normal       ---
clap->ansi_term                       0.12.1                        ---     Removed                        Normal       cfg(not(windows))
clap->atty                            0.2.14                        ---     Removed                        Normal       ---
clap->bitflags                        1.3.2                         ---     Removed                        Normal       ---
clap->strsim                          0.8.0                         ---     Removed                        Normal       ---
clap->textwrap                        0.11.0                        ---     Removed                        Normal       ---
clap->unicode-width                   0.1.12                        ---     Removed                        Normal       ---
clap->vec_map                         0.8.2                         ---     Removed                        Normal       ---
getrandom->cfg-if                     1.0.0                         ---     Removed                        Normal       ---
getrandom->libc                       0.2.154                       ---     Removed                        Normal       cfg(unix)
getrandom->wasi                       0.9.0+wasi-snapshot-preview1  ---     0.11.0+wasi-snapshot-preview1  Normal       cfg(target_os = "wasi")
getrandom->wasi                       0.9.0+wasi-snapshot-preview1  ---     Removed                        Normal       cfg(target_os = "wasi")
hermit-abi->libc                      0.2.154                       ---     Removed                        Normal       ---
num->num-bigint                       0.4.5                         ---     Removed                        Normal       ---
num->num-complex                      0.4.6                         ---     Removed                        Normal       ---
num->num-integer                      0.1.46                        ---     Removed                        Normal       ---
num->num-iter                         0.1.45                        ---     Removed                        Normal       ---
num->num-rational                     0.4.2                         ---     Removed                        Normal       ---
num->num-traits                       0.2.19                        ---     Removed                        Normal       ---
num-bigint->num-integer               0.1.46                        ---     Removed                        Normal       ---
num-bigint->num-traits                0.2.19                        ---     Removed                        Normal       ---
num-complex->num-traits               0.2.19                        ---     Removed                        Normal       ---
num-integer->num-traits               0.2.19                        ---     Removed                        Normal       ---
num-iter->autocfg                     1.3.0                         ---     Removed                        Build        ---
num-iter->num-integer                 0.1.46                        ---     Removed                        Normal       ---
num-iter->num-traits                  0.2.19                        ---     Removed                        Normal       ---
num-rational->num-bigint              0.4.5                         ---     Removed                        Normal       ---
num-rational->num-integer             0.1.46                        ---     Removed                        Normal       ---
num-rational->num-traits              0.2.19                        ---     Removed                        Normal       ---
num-traits->autocfg                   1.3.0                         ---     Removed                        Build        ---
rand                                  0.7.3                         ---     0.8.5                          Development  ---
rand->getrandom                       0.1.16                        ---     Removed                        Normal       ---
rand->rand_chacha                     0.2.2                         ---     0.3.1                          Normal       cfg(not(target_os = "emscripten"))
rand->rand_core                       0.5.1                         ---     0.6.4                          Normal       ---
rand->rand_hc                         0.2.0                         ---     Removed                        Development  ---
rand_chacha->rand_core                0.5.1                         ---     0.6.4                          Normal       ---
rand_core->getrandom                  0.1.16                        ---     0.2.15                         Normal       ---
rand_core->getrandom                  0.1.16                        ---     Removed                        Normal       ---
rand_hc->rand_core                    0.5.1                         ---     Removed                        Normal       ---
rustfft                               1.0.0                         1.0.1   6.2.0                          Normal       ---
rustfft->num                          0.4.3                         ---     Removed                        Normal       ---
textwrap->unicode-width               0.1.12                        ---     Removed                        Normal       ---
winapi->winapi-i686-pc-windows-gnu    0.4.0                         ---     Removed                        Normal       i686-pc-windows-gnu
winapi->winapi-x86_64-pc-windows-gnu  0.4.0                         ---     Removed                        Normal       x86_64-pc-windows-gnu
```

各列は以下のようになっている。

- Name: パッケージの名前
- Project: 現在プロジェクトで使用している依存関係のバージョン
- Compat: セマンティックバージョニングに従い、互換性がある最新バージョン
- Latest: 利用可能な最新バージョン
- Kind: 依存関係の種類を示す
    - Normal: 通常の依存関係
    - Development: 開発用依存関係
    - Build: ビルド用依存関係
- Platform: 依存関係がサポートするプラットフォーム

※ 各列の説明についてはよい資料が見つからなかった。そのため、ChatGPTに聞き、その後実際のコードを確認して書いたものになるので、間違っている可能性がある

## オプション

https://github.com/kbknapp/cargo-outdated?tab=readme-ov-file#options

オプションは色々あるようで、上のリンクか`cargo outdated --help`で確認ができる
