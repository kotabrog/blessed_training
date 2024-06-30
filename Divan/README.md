# Divan

https://lib.rs/crates/divan

## Divanとは

シンプルでかつ色々とできるベンチマークツール

## 参考

https://nikolaivazquez.com/blog/divan/

## Criterionとの比較

- こちらはジェネリック関数のベンチマークができる
- こちらはアロケーションの測定ができる
- よりシンプル（らしい）

一方、分析についてはCriterionの方がしっかりできそう：https://zenn.dev/kotabrog/articles/bcb72de6ad9849#html%E3%81%A7%E3%81%AE%E3%83%AC%E3%83%9D%E3%83%BC%E3%83%88

## Getting Started

https://lib.rs/crates/divan

こちらをやるのがまずはよさそう

## サンプル

色々なサンプルはこちらに

https://github.com/nvzqz/divan/tree/v0.1.0/examples/benches

## CIでの使用

https://nikolaivazquez.com/blog/divan/#benchmark-in-ci

ノイズを減らす工夫により、CIでもベンチマークが実行できるようである。  
使い方については上記リンク参照

## オプションや特殊なケースの実行方法

- ジェネリックな型：https://nikolaivazquez.com/blog/divan/#generic-benchmarks
- Bencher引数で実行方法のコントロール：https://nikolaivazquez.com/blog/divan/#benchmark-context
  - ベンチマーク関数の入力：https://nikolaivazquez.com/blog/divan/#benchmark-inputs
  - 各処理毎に処理された量のカウント：https://nikolaivazquez.com/blog/divan/#measure-throughput
    - BytesCount：バイト数をカウント
    - CharsCount：文字数をカウント
    - ItemsCount：アイテム数をカウント
- アロケートされる回数や、どれだけアロケートされたかを測定：https://nikolaivazquez.com/blog/divan/#measure-allocations
- 複数のスレッドのテスト：https://nikolaivazquez.com/blog/divan/#measure-thread-contention
- 測定方法の選択：https://nikolaivazquez.com/blog/divan/#cpu-timestamp

## 出力について

- 出力をツリー上に表示
    - 引数毎の結果が見える：https://lib.rs/crates/divan の「Getting Started」の3
    - モジュールによるグループ化：https://nikolaivazquez.com/blog/divan/#module-tree-hierarchy

## コマンドラインでの特殊な実行方法

- フィルタリング：https://nikolaivazquez.com/blog/divan/#filter-by-regex

## 測定に関する計算式

https://nikolaivazquez.com/blog/divan/#sample-size-scaling

どのような根拠で測定を行っているのか、数式や参考にした論文を載せて説明してある

## Dropについて

https://nikolaivazquez.com/blog/divan/#deferred-drop

ベンチマーク関数から値が返された場合、デストラクタはサンプルが記録されるまで実行されない

つまり、デストラクタの処理は測定されない

このようにすることで、デストラクタを測定したい場合とそうでない場合をコントロールできる