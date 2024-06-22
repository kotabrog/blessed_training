# Criterion.rs

https://lib.rs/crates/criterion

## Criterion.rsとは

統計に基づいたマイクロ・ベンチマーク・ツール

## 参考

https://bheisler.github.io/criterion.rs/book/criterion_rs.html

## チュートリアル

https://bheisler.github.io/criterion.rs/book/getting_started.html

一旦このページを実行して、使用方法について慣れるのがよさそう。

## コマンドラインへの出力について

- Warmup: 設定可能なウォームアップ期間（デフォルトでは3秒間）の間、ベンチマーク関数を自動的に反復する
- Collecting Samples
    - 指定した回数（デフォルト100回）対象の関数を実行
    - Warmup中の実行に基づいて、サンプリングにかかる時間の推定値も表示する
- time
    - 測定された１回の実行による時間に対する信頼区間が、3つの値で表示される。例：`[3.0959 ns 3.1797 ns 3.2843 ns]`
    - 左右の値はそれぞれ信頼区間の下限と上限
    - 中央の値はもっとも信頼される推定値
    - [信頼係数](https://bellcurve.jp/statistics/course/8893.html)の指定は可能
    - 信頼区間の測定のために、サンプリングを指定回数繰り返す（デフォルトは100000。つまり、Collecting Samples * time = 100 * 100000 = 10000000）
- Change
    - Criterion.rsベンチマークを実行すると、target/criterionディレクトリに統計情報が保存される
    - 前回の結果がすでにあれば、前回との差が表示される。例：`change: [-1.1233% +1.9237% +5.0838%] (p = 0.24 > 0.05)`
    - 変化がないことの閾値は設定が可能（デフォルトは）
- Detecting Outliers
    - テスト結果の外れ値として検出されたものがいくつかあるかを表示する
    - 例`Found 17 outliers among 100 measurements (17.00%), 6 (6.00%) high mild, 11 (11.00%) high severe`
    - 外れ値が多いということは、ノイズが多いということで、以下のような問題が考えられる
        - ベンチマークを実行するコンピュータに他に負荷がかかっている
        - 対象のコードの不規則性
    - 対策として、以下が考えられる
        - １回の実行毎で同じ量の作業が行われるようにテストを見直す
        - コンピュータで他の作業を行わない
        - 測定時間を長くして外れ値の影響を抑える
        - ウォームアップ時間を長くする

## コマンドラインでのオプション

https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html

## HTMLでのレポート

https://bheisler.github.io/criterion.rs/book/user_guide/html_report.html

target/criterion/report/index.htmlを見ると、ベンチマークの結果を表示するHTMLのレポートを見ることができる。

グラフなどが載っており、各グラフの説明もHTMLに記載されている

また、表示されるグラフやその保存場所については以下も参照

https://bheisler.github.io/criterion.rs/book/user_guide/plots_and_graphs.html

## ベンチマークの書き方色々

基本は「チュートリアル」の書き方をまねるのがよさそう。

その他のケースではそれぞれ以下を参照するのがよさそう

- 調査したい関数に入力があるケース：https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html
- 統計パラメータの調整：https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#configuring-sample-count--other-statistical-settings
- 一回の実行時間ではなく、スループット（単位時間当たりの処理数）を知りたい場合：https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#throughput-measurements
- プロットの軸のスケーリングを対数スケーリングなどに変えたいとき：https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#chart-axis-scaling
- 同じことをする複数の関数の比較：https://bheisler.github.io/criterion.rs/book/user_guide/comparing_functions.html
- ループ関連：https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html
- 測定方法やタイミングのtraitによるカスタマイズ：https://bheisler.github.io/criterion.rs/book/user_guide/custom_measurements.html
- ベンチマークのプロファイリングのための機能：https://bheisler.github.io/criterion.rs/book/user_guide/profiling.html
- 非同期関数のベンチマーク：https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_async.html

## cargo-criterion

https://bheisler.github.io/criterion.rs/book/cargo_criterion/cargo_criterion.html

cargo-criterionをinstallすれば、Cargoの拡張機能としてcriterionをinstallできるが、まだ実験的な状態らしい

## 特徴

上記まででできることの色々が書いてあるが、そのほかの特徴をこちらにまとめる

- ピコ秒単位からミリ秒単位まで計測でき、長時間かかるベンチマークにも対応している
    - デフォルトで適切なサンプリング方法などを切り替えてくれる
    - 手動で行う方法など詳しくはこちら：https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#sampling-mode
- 色々な制限：https://bheisler.github.io/criterion.rs/book/user_guide/known_limitations.html
- 時間の測定にはCPU時間など色々とあるが、Criterionでは[wall-clock time](https://en.wikipedia.org/wiki/Elapsed_real_time)（純粋な経過時間）が使用されている

## ベンチマークのためのデータ取得方法と分析プロセスについての詳細

https://bheisler.github.io/criterion.rs/book/analysis.html

## CIでの使用について

推奨されていない：https://bheisler.github.io/criterion.rs/book/faq.html#how-should-i-run-criterionrs-benchmarks-in-a-ci-pipeline

これは、CI環境ではノイズが入りやすく、正しい測定ができないことが多いからである

しかし、かわりにIaiというクレートを紹介している：https://bheisler.github.io/criterion.rs/book/iai/iai.html
