# hyperfine

https://github.com/sharkdp/hyperfine#hyperfine

## hyperfineとは

Rust製のベンチマークツール

## Rust製のツール

こちらは（見逃していなければ）Rust内で使用するライブラリではなく、Rust製のツールである。

## 登録が必要？

https://github.com/sharkdp/hyperfine?tab=readme-ov-file#sponsors

webページにとんで、start nowをおすと登録ページが出てくる

おそらく登録が必要なのは、CI環境の提供についてのみと思われる

## install

https://github.com/sharkdp/hyperfine?tab=readme-ov-file#on-ubuntu

いろいろな環境へのいろいろなinstall方法がある。

cargoを使用する場合以下

```sh
$ cargo install --locked hyperfine
```

## 使い方

https://github.com/sharkdp/hyperfine?tab=readme-ov-file#usage

ベンチマークを実行するには、以下のように`hyperfine <command>...`とする。

```sh
$ hyperfine 'sleep 0.3'
```

## 引数について

https://github.com/sharkdp/hyperfine?tab=readme-ov-file#usage

- runs（-r）: 実行回数を指定できる（デフォルトは10回）
  - 例：`hyperfine --runs 5 'sleep 0.3'`
- 複数のコマンドの確認（比較もしてくれる）
  - 例：`hyperfine 'hexdump file' 'xxd file'`
- warmup（-w）: 事前に指定した回数実行する
  - 使用ケース（warm cache）について：https://github.com/sharkdp/hyperfine?tab=readme-ov-file#warmup-runs-and-preparation-commands
  - 例：`hyperfine --warmup 3 'grep -R TODO *'`
- prepare（-p）：各テストの前に指定したコマンドを実行する
  - 使用ケース（cold cache）について：https://github.com/sharkdp/hyperfine?tab=readme-ov-file#warmup-runs-and-preparation-commands
  - 例：`hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' 'grep -R TODO *'`
- parameter-scan（-P）：パラメータを変化させて実行
  - 例：`hyperfine --prepare 'make clean' --parameter-scan num_threads 1 12 'make -j {num_threads}'`
- parameter-step-size（-D）：間隔を指定してパラメータを変化させて実行（小数も可）
  - 例：`hyperfine --parameter-scan delay 0.3 0.7 -D 0.2 'sleep {delay}'` （これで、delay=0.3, 0.5, 0.7で実行される）
- parameter-list（-L）：指定したパラメータのリストで実行（数字以外も可）
  - 例：`hyperfine -L compiler gcc,clang '{compiler} -O2 main.cpp'`
- shell（-S）：シェルの指定（デフォルトは環境によって決まっている）
  - `hyperfine --shell zsh 'for i in {1..10000}; do echo test; done'`
- --shell=none（-N）：シェルの呼び出しなしで実行する場合に指定
  - シェルの起動時間は補正されるが、それでもノイズが気になるような、5ms未満ほどの高速なコマンドの場合に指定
  - 例：`hyperfine -N 'grep TODO /home/user'`
- --export-markdown：`--export-markdown <file>`でマークダウン形式で出力
- --export-csv: `--export-csv <file>`でcsvとして
- --export-json: `--export-json <file>`でjsonとして
- --help: そのほかの引数は`--help`で確認できる

## そのほかの使い方

- シェル関数とエイリアス：https://github.com/sharkdp/hyperfine?tab=readme-ov-file#shell-functions-and-aliases
- JSON形式で出力すれば、サンプルとしてある解析プログラムを使用できる：https://github.com/sharkdp/hyperfine?tab=readme-ov-file#json
