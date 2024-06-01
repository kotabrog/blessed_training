# cargo-nextest

https://nexte.st/

## cargo-nextestとは

cargo testをより進化させたテストランナー

## install

https://nexte.st/book/installation

上記を参照

Linux x86_64であれば以下を実行

```sh
$ curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
```

## 実行方法

https://nexte.st/#quick-start

installされていれば以下で実行できる

```
$ cargo nextest run
```

## CIへの導入

https://nexte.st/book/pre-built-binaries#using-pre-built-binaries-in-ci

## Update

https://nexte.st/book/updating#updating-nextest

以下でアップデートができる

```sh
$ cargo nextest self update
```

## 制限

https://nexte.st/book/usage#limitations

- 個々のテストは別々のプロセスとして実行される（cargo testと一緒）
- 特定のテストバイナリをテストから除外する方法がない
- Doctestはサポートされていない
    - cargo testでは`cargo test --doc`


## 機能詳細

https://nexte.st/

- cargo testのオプションは基本対応してる
    - skip, exactオプションは、かわりにより強力なフィルター機能がある（後述）
- デフォルトでは、テスト出力をキャプチャし、失敗時のみ表示する
    - テスト出力をキャプチャしたくない場合は`--no-capture`をつける。このモードではテストを連続的に実行する
- `cargo nextest list`でテストをビルドしてリストアップする： https://nexte.st/book/listing
- `--retries`をつけることで、失敗したときに再実行することができる：https://nexte.st/book/retries
- SLOW判定はデフォルトで60秒だが、タイムアウトで終了することはない（多分。後に例がある）：https://nexte.st/book/slow-tests
    - 例えば設定で`slow-timeout = { period = "30s", terminate-after = 4 }`とすると、30秒ごとにSLOWと表示され、4回目にテストがTIMEOUTになる
    - デフォルトだとタイムアウト時には、そのテストのプロセスにSIGTERMが送られ、それでも終了しない場合その10秒後にSIGKILLが送られる
    - SIGKILLが送られるタイミングも設定できる
    - テストごとに時間の設定は変えることができる
- テスト内でサブプロセスを作成し、それがリークした場合に（すべてではないが）検出ができる：https://nexte.st/book/leaky-tests
    - 今のところ、検出できるのは、テストから標準出力か標準エラーを継承するサブプロセスのみ
    - リークした場合もテストはパスしたものとみなされる（LEAKと表示はされる）
    - テスト終了から標準出力・標準エラーがクローズされるまで指定した時間待つ（デフォルトでは100ms）が、この時間も設定できる
- テストをフィルタリングするためのドメイン固有言語（DSL）があり、それを用いてどのテストを実行するかフィルタリングできる：https://nexte.st/book/filter-expressions
    - `-E` に続けてDSLを書いて指定する
    - 複数指定が可能で、どれかにあてはまったら実行される
        - `-E 'test(my_test)' -E 'package(my-crate)'`で、my_testという文字列を含むテストと、my-crateパッケージに含まれるテストが実行される
        - これは`-E 'test(my_test) + package(my-crate)'`と書くこともできる
    - 正規表現も使える
    - そのほか色々な表現ができるので詳細は公式ドキュメントを参照：https://nexte.st/book/filter-expressions#dsl-reference
- テストのビルドと実行を切り離すことができる：https://nexte.st/book/reusing-builds
    - あるマシンでビルドをアーカイブし、そのアーカイブを展開して別のマシンでテストすることができる
    - 例えば以下のケースなどで有用
        - クロスコンパイルのテスト
        - パーティショニング：ビルドマシンでビルドした後に複数のターゲットマシンでテスト実行を分割する
        - より貴重な資源を節約：GPUが接続されたマシンは実行時のみなど
- `--partition`でテストをパーティショニング・シャーディングできる：https://nexte.st/book/partitioning
    - CIで別々のマシンでテストを実行ができる
- ターゲットランナーをサポートしている：https://nexte.st/book/target-runners
    - 特定のターゲットプラットフォーム向けにビルドしたバイナリを実行するための外部プログラムやスクリプトを指定できる
    - クロスコンパイルする場合などのテストが可能
- その他のオプション：https://nexte.st/book/other-options
    - `--no-fail-fast`
        - デフォルトだと、テストが一つでも失敗すると、それ以降新しいテストを実行しない（`-j`オプションと組み合わせると、この現象を確認しやすい）
        - このフラグをつけると、失敗しても、すべてのテストを実行する
    - `-j`, `--test-threads`：同時に実行するテストの数
    - オプションの一覧：https://nexte.st/book/running#options-and-arguments
- アウトプットの制御（JSON形式で出力など）：https://nexte.st/book/machine-readable
- 設定ファイルについて：https://nexte.st/book/configuration
- 他のツールとの組み合わせ：https://nexte.st/book/integrations

以下はデフォルト設定でテストを実行した場合で、test3はテスト内に無限ループがある（423.379sの時点でCtrl+Cを押した）

```sh
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
    Starting 3 tests across 1 binary (run ID: c5fa662c-2cf5-4b07-9e1c-39cd6922c913, nextest profile: default)
        PASS [   0.033s] test_nextest::bin/test_nextest tests::test2
        PASS [   0.033s] test_nextest::bin/test_nextest tests::test1
        SLOW [> 60.000s] test_nextest::bin/test_nextest tests::test3
        SLOW [>120.000s] test_nextest::bin/test_nextest tests::test3
        SLOW [>180.000s] test_nextest::bin/test_nextest tests::test3
        SLOW [>240.000s] test_nextest::bin/test_nextest tests::test3
        SLOW [>300.000s] test_nextest::bin/test_nextest tests::test3
        SLOW [>360.000s] test_nextest::bin/test_nextest tests::test3
        SLOW [>420.000s] test_nextest::bin/test_nextest tests::test3
     Running [ 00:07:03] [========================================================================================================================================================>                                                                           ] 2/3: 1 running, 2 passed, 0 skipped     
   Canceling due to interrupt: 1 test still running
      SIGINT [ 423.379s] test_nextest::bin/test_nextest tests::test3
------------
     Summary [ 423.379s] 3 tests run: 2 passed, 1 failed, 0 skipped
error: test run failed
```

