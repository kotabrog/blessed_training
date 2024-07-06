# flamegraph

https://github.com/flamegraph-rs/flamegraph#cargo-flamegraph

## flamegraphとは

Rust製のフレームグラフ・ジェネレータ。

## フレームグラフとは？

https://deeeet.com/writing/2016/05/29/go-flame-graph/

こちらが参考になった。

また、flamegraphのリポジトリ内では以下で説明がある。

https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#systems-performance-work-guided-by-flamegraphs

また、flamegraphは、Linuxではperfを、それ以外ではdtraceを使用して解析を行い、その結果からSVGを作成している。

この解析を行うツールであるperfについては以下が参考になった。

https://zenn.dev/termoshtt/articles/perf-tutorial1

## install （WSL2環境）

https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#installation

基本的には以下でinstallを行う。

```sh
cargo install flamegraph
```

ただし、Linuxではperfを、それ以外ではdtraceを使用するため、これらのツールが環境に入っている必要がある。

詳細は上記URL参照。

以下は、WSL2での環境構築方法になる。

まず、WSL2では、apt-getなどを使用する方法ではprefをインストールすることができないため、以下の記事のようにする必要がある。

https://qiita.com/neko_the_shadow/items/4aef8dc500b2450c938b

途中で以下のようなエラーが出るかもしれない。

```sh
fatal error: Python.h: No such file or directory
    2 | #include <Python.h>
      |          ^~~~~~~~~~
compilation terminated.
error: command '/usr/bin/gcc' failed with exit code 1
cp: cannot stat 'python_ext_build/lib/perf*.so': No such file or directory
make[2]: *** [Makefile.perf:632: python/perf.so] Error 1
make[1]: *** [Makefile.perf:238: sub-make] Error 2
make: *** [Makefile:70: all] Error 
```

この場合は以下の記事のように行う。

https://zenn.dev/bluepost/articles/a824b2905df36f

（私の環境ではpython3.9-devをinstallすれば動いた。-Iオプションでの指定は必要ない）

## CPUイベントへのアクセス権限

https://zenn.dev/termoshtt/articles/perf-tutorial1#%E9%9D%9E%E7%89%B9%E6%A8%A9%E3%83%A6%E3%83%BC%E3%82%B6%E3%83%BC%E3%81%8B%E3%82%89cpu%E3%82%A4%E3%83%99%E3%83%B3%E3%83%88%E3%81%B8%E3%81%AE%E3%82%A2%E3%82%AF%E3%82%BB%E3%82%B9%E3%82%92%E8%A8%B1%E5%8F%AF%E3%81%99%E3%82%8B

CPUイベントへのアクセス権限がないと、perfによる解析ができないため、flamegraph使用時にエラーになる。

flamegraphを使用する際に、上記リンクの方法で一時的に設定を上書きするのがいいかもしれない。

（私の環境では、とりあえず値を 1 にしたところflamegraphを動かすことができたが、デフォルトの 2 のままだとエラーになった）

## 使い方

https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#examples

直接flamegraphを使う方法とcargoを使う方法がある。

```sh
$ flamegraph -- file
$ cargo flamegraph
```

直接flamegraphを使用する場合は、実行するファイルを上記のように指定する。

一方cargoを使用する場合は、上記の書き方で`cargo run --release`を実行した場合の解析ができる。

できることは基本同じだが、cargoを使用した方法の場合、例えば以下のようなことができる。

- `--dev`をつけると開発用のビルドが行われる
- `--bin=file`か`--bin file`で`src/bin/file`を実行できる
- `--bench benchmark`で`benches/benchmark`を実行できる
- `--example example`で`examples/example`を実行できる
- `--unit-test test`で単体テストを実行できる
  - 色々試したがうまく実行できなかった
  - 参考：https://stackoverflow.com/questions/77484315/how-do-you-actually-run-cargo-flamegraph-on-tests
- `--test test_name`で統合テスト（`tests/test_name`）を実行できる

また、両方に共通することだと、以下などのオプションがある

- `-o output_name.svg`で出力ファイル名を変えられる（デフォルトは`flamegraph.svg`）
- `--no-inline`でperfが行うどの関数がインライン化されているかの計算を停止する（これを行うのに結構時間がかかるので不必要ならオフにできる）
- `-- arguments`でcargo runに渡す引数を指定できる
  - 直接flamegraphを実行の場合は`flamegraph -- file arguments`
- `-c "option1 option2"`でperf/dtraceに`"option1 option2"`をわたすことができる

## その他の参考

https://zenn.dev/termoshtt/books/b4bce1b9ea5e6853cb07/viewer/flamegraph
