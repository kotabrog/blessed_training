# rand

https://lib.rs/crates/rand

## randとは

乱数を生成するための色々な機能がまとめられたクレート。

## この記事の構成

version 0.8.5のrandについて調査した。

全体の構成としては以下のようになっている。

- 乱数とは（ちょっとだけ）
- randクレート群の構成
- 各クレート、各機能の詳細な説明（めちゃくちゃある）

そのため、ちょっとだけrandクレートについて知りたいなという場合には、あまり向かない記事になっている。

逆に、randクレートについて全部知りたいんだ！という場合には、ある程度内部でどのように実装されているのかも含めてまとめたため、参考になるかもしれない。

## 乱数について

### 乱数、疑似乱数、安全な乱数

そもそも乱数生成とは、という点から考えていく。

https://docs.rs/rand/latest/rand/rngs/index.html

「真の」乱数生成器（TRNG）は、予測困難なデータソースを使用して、ランダムなビット列を取得し、なんやかんやして、乱数を生成する。  
予測困難なデータソースとは、ハードウェアから発生する物理特性に基づいた不規則な環境ノイズなどのことである。

これは例えばオペレーティングシステムによる乱数生成などがあてはまる。

https://utakamo.com/article/linux/architecture/directory/dev/random-urandom.html

「疑似」乱数発生器（PRNG）はアルゴリズムを使ってシードを疑似乱数列に変換する。

アルゴリズムを使用することで、高速で、よく分散した予測不可能な乱数を生成することもできる。

また、通常は決定論的であり、アルゴリズムとシードが与えられれば、乱数列は完全に再現可能である。

さらに、「暗号的に安全な」擬似乱数生成器（CSPRNG）は、PRNGのうちより安全なもののことである。  
生成器の安全性は、内部状態の隠蔽と強力なアルゴリズムの使用の両方に依存する。

### TRNGの使いどころ

TRNGは上記のような性質のものなので、そのまま利用するには以下のデメリットがある。

- 効率性: OSの機能なので、PRNGに比べるとかなり低速である
- 再現性: 決定論的ではないため再現性がない

そのため、TRNGはシードの生成に使われることがよくある。

## randの構成

### randの特徴

- 小さくてシンプルではない
  - 小さくてシンプルなクレートを求めている人に対しては[fastrand](https://lib.rs/crates/fastrand)や[ooradom](https://lib.rs/crates/oorandom)を勧めている
- 正しさ、スピード、柔軟性に重点を置いている

### randクレートの構成

randクレートでは以下の2つに大きく分けることができる。

- 乱数の生成: 具体的なPRNGの実装など
- 乱数をもとに具体的な確率分布にそった乱数を生成: 範囲を絞った一様分布や、ベルヌーイ分布（確率pでtrueになる分布）など

例えば、PRNGを使用して乱数を生成しても、それはu32やu64などの決まった型なため、適切な分布をセットで与えて、欲しい型や欲しい確率分布での乱数を生成するようになっている。  
（もちろん、PRNGと分布がセットになって、より使いやすくなっている関数や構造体もある）

### rand crate群について

https://rust-random.github.io/book/crates.html

すべてのユーザーがすべての機能が欲しいわけではないため、randはすべての機能がrandクレートにあるわけではなく、いくつかのクレートにわかれている。

```
                                           ┌ statrs
getrandom ┐                                ├ rand_distr
          └ rand_core ┬─────────────┬ rand ┘
                      ├ rand_chacha ┘
                      ├ rand_pcg
                      └ [other RNG crates]
```

- getrandom
  - OSの乱数発生器にアクセスするためのインターフェース（TRNG）
  - シードの生成に利用
- rand_core
  - PRNGのコアになっている部分
  - PRNGに実装すべきトレイトなどが定義されている
  - シードの生成などにgetrandomを使用
- 各PRNG（rand_chacha,rand_pcgなど）（本記事では扱わない）
  - rand_coreのトレイトを実装した具体的なPRNG
- rand
  - PRNGをただ使用したいだけであれば基本的にこのクレートだけを使用する
  - rand_coreのトレイトを実装したPRNGに対して拡張的な機能が実装されている
  - 確率分布関連のトレイトや具体的な実装がある
  - 基本的なPRNGが実装されている
- rand_distr, statrs（本記事では扱わない）
  - 色々な分布の実装がある

## getrandomについて

https://docs.rs/getrandom/latest/getrandom/

### getrandomとは

getrandomクレートは、OSの乱数発生器にアクセスするためのインターフェースである。

### 使用方法

基本的な使用方法としては、`getrandom::getrandom`関数で、OSの乱数発生器を使用して乱数を生成するのみである。

### 使いどころ

https://rust-random.github.io/book/crate-platforms.html

このクレートは基本的にrand使用者が直接触れることはない。  
例えば、自分で乱数生成器を作成する場合も、基本的には使用しない。  
低レベルのライブラリなどを開発する場合などには直接使用する場合もあるようである。

TRNGの使いどころで述べたように、getrandomの使いどころとしては、シード生成部分などである。

そのため、後述するrand_core::SeedableRng::from_entropyでgetrandomは使用されている。

また、後述するrand_core::OsRngは、getrandomの薄いラッパーのような構造体で、その他の様々なPRNGと同様に使用することができるようになっている。

### linuxの場合とエラーが起こるケースについて

getrandomは滅多にエラーにはならないようだが、方針としては、セキュアな乱数を生成できないのであれば、エラーにするような方針のようである。

例えばlinuxの場合、getrandomによって/dev/random、/dev/urandomが呼び出される。（詳細は以下参照）

https://utakamo.com/article/linux/architecture/directory/dev/random-urandom.html

/dev/randomは、エントロピープール（詳細は上記リンク参照）を元に乱数を生成するようであるが、それが空になるとロックされてしまう。  
また、/dev/urandomはエントロピープールが空になった場合でも、再利用することで基本的にはエラーにならないようである。

getrandomを使用すると基本的には/dev/randomが呼ばれるが、これがエラーになった場合には、/dev/urandomが呼ばれるようになっている。  
このフォールバックをオフにすることもできるようで（以下参照）、フォールバックをオフにするとエラーになる可能性もありそうである。

https://docs.rs/getrandom/latest/getrandom/#devurandom-fallback-on-linux-and-android

また、getrandomはエラーが起こる可能性があるのでResult型で返すようになっているが、それを使用しているrandクレートでは、一部Resultで返すものもあるものの、基本的にResult型を使用せず、getrandomが失敗したらpanicを呼ぶようにしているようである。

https://rust-random.github.io/book/guide-err.html

※ linux以外のサポートターゲットについてはこちらを参照

https://docs.rs/getrandom/latest/getrandom/#supported-targets

### 制限

このクレートはデフォルトの状態だと、randでも有効になっているが、例えばstdを使用しない場合などには基本的に使用することができない。

また、WebAssembly（wasm32-unknown-unknown ターゲット）は自動的にはサポートされていないため、使用したい場合には適切な対処が必要である。

https://docs.rs/getrandom/latest/getrandom/#webassembly-support

また、ブートプロセスの初期段階では、OSがRNGを安全にシードするのに十分なエントロピーを収集していないことがあり（上記のlinuxの例を参照）、その場合うまく動作しない（エラーになったりなど）ことがある。

https://docs.rs/getrandom/latest/getrandom/#early-boot

## rand_coreについて

https://docs.rs/rand_core/latest/rand_core/index.html

### rand_coreとは

randやその他の乱数関連のクレートの基盤になっている、randクレート群のコア部分。

以下の3つの重要なトレイトが実装されている。

- RngCore: PRNGに実装する必要があるトレイト。これを実装していると、randクレートによって色々な拡張機能が使えるようになる
- SeedableRng: 明示的にシードを設定できるPRNGに実装するトレイト
- CryptoRng: PRNGが安全であることを示すマーカー・トレイト

また、加えて以下が実装されている。

- BlockRngCoreトレイト: ブロックベースのPRNGに実装するトレイト
- BlockRng構造体: BlockRngCoreトレイトを実装した型から自然に作成できるRngCoreを実装した構造体
- OsRng構造体: getrandomの薄いラッパーでRngCoreを実装している

PRNGを「自分で」実装したい場合に使用したい機能が集まっており、またこれらのトレイトの拡張としてrandでは色々な機能が実装されているため、まさにrandクレート群のコア部分といえる。

### RngCoreトレイト

https://docs.rs/rand_core/latest/rand_core/trait.RngCore.html

もっとも中心的な機能がRngCoreトレイトである。

このトレイトは、すべての乱数生成器が実装すべき基本的なインターフェースを定義している。

また、このトレイトを実装していることで、randクレートの様々な機能を使用することができるようになる。

そのため、乱数生成器をただ使用したいだけの場合は、このクレートを実装すると自動的に実装される、より拡張されたrandクレートのRngトレイトを使用することになる。

RngCoreトレイトには以下の4つのメソッドが定義されている。

- next_u32: 32ビットの乱数を生成する
- next_u64: 64ビットの乱数を生成する
- fill_bytes: 任意のバイト列を乱数で埋める
- try_fill_bytes: Resultを返すfill_bytes

基本的には上の3つのいずれかを実装し、それ以外については、実装したものを使用するか、もしくはrand_core::implsのヘルパー関数を使用する方法があるようである。（詳細は上記リンク参照）

また、基本的にはResultでは返さないが、try_fill_bytesのみResultで返している。

上記リンクでは、その他、例えばRngCoreトレイトを実装する場合、Debugトレイトは内部情報を一切出さないものにするであるとか、Copyトレイトは決して実装してはいけないことなど、自分で乱数生成器を作成するときのことを考えていくつかの注意も書いている。

### SeedableRngトレイト

https://docs.rs/rand_core/latest/rand_core/trait.SeedableRng.html

明示的にシードを設定できる乱数生成器に実装するトレイト。  
逆にいえば、このトレイトを実装していない場合、シードの指定はできない。

このトレイトでは以下を定義する必要がある。

- Seed type
  - 内部で保持するシードの型
  - Sized + Default + AsMut<[u8]> が求められる（[u8; 32]など）
  - 通常のPRNGだと100 bitsほど、つまり [u8; 12] ほどが求められるが、より安全である必要があるCSPRNGには256ビットのシード、つまり[u8; 32]が推奨されている。
- from_seedメソッド
  - 与えられたシードを用いて新しいPRNGを作成する
  - PRNGの実装では、シードのビットがうまく分散していると仮定することが許されている
  - 特に断りのない限り、同じシードであれば同じ値を返すように、再現可能であるべき
  - パニックにならないようにすべき

また、提供されているメソッドとして以下がある。

- seed_from_u64
  - u64からPRNGを作成できる
  - シードのビットはうまく分散していなければならないが、例えば0,1,2などの単純な値を渡しても、それを満たすようにいい感じに変換してくれる
  - そういった機能があるため、基本的にはデフォルトの実装が推奨されている
  - 64ビットだけなので、暗号化には適していない
- from_rng
  - 別のPRNGを渡して、それで生成したシードを元にPRNGを作成する
  - 再現可能であるべき
  - メインのPRNGから、多数のPRNGを作成したい場合などに有用
  - メインのPRNGは少なくとも子PRNGと同程度には高品質であるべきで、相関を避けるためにメインと子は異なるアルゴリズムであることが推奨される
- from_entropy
  - getrandomによって取得した値をシードとしてPRNGを作成する
  - getrandomは再現可能でないので、このメソッドによって作成するアプローチは再現可能でない（非決定論的）

### BlockRngCoreトレイトとBlockRng構造体

https://docs.rs/rand_core/latest/rand_core/block/index.html

BlockRngCoreトレイトは、ブロックベースの乱数生成器（つまり、まとめて作成するもの）に実装するトレイトである。

まとめて乱数を生成したい場合に有用である。  
また、CSPRNGの性能を上げたい目的で使用される場合もある。

このトレイトでは以下を定義する必要がある。

- Item type: 生成する乱数の型（u32など）
- Results type
  - 乱数のブロックの型
  - AsRef<[Self::Item]> + AsMut<[Self::Item]> + Default が求められる（[u32; 16]など）
- generateメソッド
  - 乱数のブロックを生成するメソッド

また、BlockRng構造体は、Resultsがu32配列であるBlockRngCoreを実装した型に対する、RngCoreを実装したラッパーである。

BlockRng構造体はBlockRngCoreトレイトのgenerateメソッドを使用して生成した乱数のブロックを使用して、RngCoreトレイトのnext_u32メソッドやfill_bytesメソッドなどを、効率的に実装している。  
そのため、BlockRngCoreトレイトを実装していれば、自分でRngCoreを実装する必要は基本的にない。

fieldやメソッドなどは以下のようになっている。  
newメソッド以外は、RngCoreの実装のためにあるようなので、直接触ることは基本的になさそう。

- core field: BlockRngCoreを実装した乱数生成器
- new メソッド: BlockRngCoreを実装した乱数生成器から新しいBlockRngを作成する。（core fieldに渡した乱数生成器がセットされる）
- index メソッド
  - privateなindex field（usize）を取得するメソッド
  - coreで生成したブロックのどこまで使用したかを表す（indexがブロックと同じかそれ以上の場合、ブロックをすべて消費したことを表す）
  - おそらく直接呼び出すことは想定されていない
- generate_and_set メソッド
  - ブロックを新しく生成し、indexを指定した値にセットする
  - まだ消費されていない乱数があるとassertされる
  - おそらく直接呼び出すことは想定されていない
- reset メソッド
  - 現在のブロックをすべて消費したことにする（内部的にはindexをブロックと同じ長さにする）
  - おそらく直接呼び出すことは想定されていない

また、RngCore、SeedableRng、CryptoRng（後述）や、Clone、Debugなどの基本的なトレイトが実装されている。

また、Resultsがu64配列であるBlockRngCore用のBlockRng64もある。

### CryptoRngトレイト

https://docs.rs/rand_core/latest/rand_core/trait.CryptoRng.html

CryptoRngトレイトは、乱数生成器が暗号学的に安全であることを示すマーカー・トレイトである。  
つまり、このトレイトによるメソッドなどはない。

このトレイトはもちろん暗号学的に安全であることを保証するものではない。  
そのため、十分に評価されたアルゴリズムで、十分に評価されたコードにのみ、このトレイトは実装されるべきである。

また、RngCore と CryptoRng が実装された型に自動的に実装される拡張traitとしてCryptoRngCoreがある。

CryptoRngCoreにはas_rngcore メソッドという、RngCore へのアップキャストを行うメソッドのみ実装されている。

### OsRng構造体

https://docs.rs/rand_core/latest/rand_core/struct.OsRng.html

rand_coreクレートには具体的な乱数生成器が一つだけ実装されている。  
それがOsRngである。

これは、getrandomのラッパーのような乱数生成器である。

乱数の生成にgetrandomを使用するため、構造体のサイズは0である。

getrandomについては「getrandomについて」を参照。

Clone、Debugなどの基本的なトレイトの他、RngCoreとCryptoRngが実装されている。

## randについて

https://docs.rs/rand/latest/rand/index.html

### 全体の構成について

randクレートには、rand_coreを基礎として、様々な機能が実装されている。  
また、一部rand_coreにあるものをre-export（再公開）しているものもある。

おおまかには以下のような構成になっている。

- Rng: RngCoreの拡張トレイトで、基本的にrandを使用したい場合はPRNGに定義されているこれを使用する
- distributionモジュール: 確率分布関連のモジュール
- seqモジュール: シーケンス関連の機能が実装されているモジュール
- rngsモジュール: RngCoreを実装した基本的なPRNGが実装されている
- 使いやすい関数
  - random: 乱数を一つ作成
  - thread_rng: シード値が定期的にリセットされるPRNGを取得できる

### Rngトレイト

https://docs.rs/rand/latest/rand/trait.Rng.html

RngCore に自動的に実装される拡張traitで、乱数生成に関する色々なメソッドが定義されている。

randクレートの主要なtraitで、乱数を生成する目的でrandクレートを使用するのであれば基本的にこのtraitを使用する。

以下のメソッドが定義されている。

- gen
  - `gen<T>(&mut self) -> T where Standard: Distribution<T>` である（Standard, Distributionについては後述）
  - Standard（一般的には一様分布。詳細は後述）にそって、型 T でランダムな値を作成
  - 内部的にはStandard.sampleを呼んでいる
  - Standard構造体が実装されていればいいので、例えばu32などの単体の値を生成することや、(f64, f32, u32)などの異なる型が混じったtuple、また配列なども生成することができる
  - 整数の配列の場合、特に要素の型が小さい（64ビット未満）場合は、Rng::fillを使った方が速い（Standardの実装の問題）
- gen_range
  - `gen_range<T, R>(&mut self, range: R) -> T where T: SampleUniform, R: SampleRange<T>`である（SampleUniform, SampleRangeについては後述）
  - 与えられた範囲内で無作為な値を生成する
  - 内部的にはrange.sample_single（SampleRangeは与えられた範囲内の数値を一様にサンプリングするためのトレイト。詳細は後述）が呼ばれている
  - この関数は、指定された範囲から1回だけサンプリングする場合に最適化されている。
    - 同じ範囲から繰り返しサンプリングする場合は、より高速な一様分布の型であるUniformがある（後述）
  - 値の指定方法は`gen_range(low..high)` と `gen_range(low..=high)`
- sample
  - `sample<T, D: Distribution<T>>(&mut self, distr: D) -> T`である。（Distributionについては後述）
  - 与えられた分布 distr を使用して、新しい値をサンプリングする
  - 内部的にはdistr.sample（Distributionは分布に関するトレイト。上記のStandardやUniformもこのトレイトを実装している。後述）を呼んでいる
- sample_iter
  - `sample_iter<T, D>(self, distr: D) -> DistIter<D, Self, T> where D: Distribution<T>, Self: Sized`である。（Distribution, DistIterについては後述）
  - 与えられた分布を使用して値を生成するイテレータを作成する
  - 内部的にはdistr.sample_iterを呼んでいる
- fill
  - `fill<T: Fill + ?Sized>(&mut self, dest: &mut T)`である。（Fillについては後述）
  - Fillトレイト（ランダムな値で埋めることに関してのトレイト。後述）を実装した型のdestをランダムなデータで埋める
  - 内部的にはdest.try_fillを呼んでいる
  - try_fillと同じ処理だが、エラー時にパニックになる。
- try_fill: fillと同じ処理だが、Result型を返す
- gen_bool
  - `gen_bool(&mut self, p: f64) -> bool`である
  - 確率 p でtrueを返す
  - 内部的にはBernoulli（ベルヌーイ分布。詳細は後述）を使用している
  - 同じ確率から繰り返しサンプリングする場合は、Bernoulliを直接使用した方が早いかもしれない（このメソッドではBernoulliを毎回インスタンス化しているためと思われる）
- gen_ratio
  - `gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool`である
  - 確率 numerator / denominator でtrueを返す
  - その他についてはgen_boolと同様

### Fillトレイト

https://docs.rs/rand/latest/rand/trait.Fill.html

ランダムな値で配列などを埋める場合に実装するトレイト。

例えば[u32]や[f64]などに対して実装されており、基本的な数値型の配列には実装されている。

Rngトレイトを経由して（Rng::fill, Rng::try_fill）使うことがおそらく想定されており、直接Fillトレイトを使用することは基本的にない。  
例えば、自前の数値型に対してFillトレイトを実装したい場合などに直接使用すると思われる。

以下のメソッドを実装する必要がある。

- try_fill: Rngによって自分自身を埋めるメソッド

### distributionモジュール

https://docs.rs/rand/latest/rand/distributions/index.html

確率分布と、そこからランダムな値を取得するためのモジュールである。

Distributionトレイトや、Distributionトレイトを実装したいくつかの確率分布が実装されている。

Rngのメソッドの内部実装は基本的にこのモジュールで実装されている。

概要としては以下のようになっている。

- Distributionトレイト: 分布に関するトレイト。Rngを元に分布にそった乱数を生成するためのもの
- Distributionを実装していると自動的に作成
  - DistIter構造体: Distributionを実装した構造体の分布の値を生成するイテレータ
  - DistMap構造体: もとの分布にクロージャを通してできる分布
- DistStringトレイト: Stringに関する分布のトレイト
- Distributionが実装されている分布の構造体
  - Standard: もっとも基本的な分布で、基本的には全体に対する一様分布
  - Open01, OpenClosed01構造体: 浮動小数点の開区間`(0, 1)`や半開区間`(0, 1]`に対する一様分布
  - Uniform: 範囲内の一様分布
  - Bernoulli: 確率を指定してbool値を出力
  - Alphanumeric: a-z、A-Z、0-9に対する一様分布
  - Slice: スライス（配列など）からランダム（一様）に値を取得
  - WeightedIndex: 離散項目の重み付きサンプリング

#### Distributionトレイト

https://docs.rs/rand/latest/rand/distributions/trait.Distribution.html

ランダムな値を分布にそって生成するためのトレイト。

`distr.sample(&mut・rng)`と`rng.sample(distr)`によって、DistributionとRngの両トレイトから、分布にそったサンプリングをすることができる。

Distributionトレイトを実装する場合、その構造体自体は不変であることが期待されている。  
つまり、内部に可変な状態（ステイト）を持たず、Rngを実装した構造体の出力のみに依存して、分布からサンプリングされるアルゴリズムであることが期待されている。  
そのため、Distributionトレイトを実装した構造体は、基本的にスレッドセーフである。

また、基本的には、固定シードのPRNGを使用した場合、再現可能であることが期待される。

自分でDistributionトレイトを実装する場合は、以下のメソッドを実装する必要がある。

- sample
  - `sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T`である
  - rngを乱数源として、Tの乱数値を生成する。
  - Rng::sampleではこのメソッドが内部で呼ばれている

また以下のメソッドが提供されている。

- sample_iter
  - `sample_iter<R>(self, rng: R) -> DistIter<Self, R, T> where R: Rng, Self: Sized`である
  - rngを乱数源として、Tの乱数値を生成するイテレータを作成する
  - DistIter構造体については後述
- map
  - `map<F, S>(self, func: F) -> DistMap<Self, F, T, S> where F: Fn(T) -> S, Self: Sized`である
  - TをSelfで生成して、それをfunc（クロージャ）で変換したS型の値を返す分布を作成する
    - 例えばUniformで1～6の値を出力できるようにし、クロージャとして偶数ならTrueを返すものを渡せば、1/2でTrueを返す分布を作成できる
    - DistMap構造体については後述

#### DistIter構造体

https://docs.rs/rand/latest/rand/distributions/struct.DistIter.html

Rngを実装した構造体を乱数源として使用し、Distributionを実装した構造体の分布の値を生成するイテレータ。

Iteratorトレイトが実装されているため、Iteratorとして使用できる。

この構造体は、基本的にDistributionのsample_iterメソッドによって生成する。

#### DistMap構造体

https://docs.rs/rand/latest/rand/distributions/struct.DistMap.html

Distributionから得た値をクロージャを通して別の値（別の型）にして出力を得る分布の構造体。

この構造体にもDistributionが実装されているため、DistMap自体もDistributionのように使用することができる。

この構造体は、基本的にDistribution::mapメソッドによって生成する。

#### DistStringトレイト

https://docs.rs/rand/latest/rand/distributions/trait.DistString.html

Stringに関するランダムサンプリングのトレイト。

rustクレートではStandardとAlphanumericにのみ実装されている。（それぞれについては後述）

このトレイトを実装したい場合、以下を実装する必要がある。

- append_string
  - `append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize)`である。
  - stringにlen分だけランダムに文字を追加する

Distributionトレイトは生成するだけであるが、こちらのトレイトはStringに文字を追加する点が異なる。

以下のメソッドが提供されている。

- sample_string
  - 指定した長さのStringを作成すす
  - デフォルト実装だと、新しいStringを作成して、そこにappend_stringをしている

#### Standard構造体

https://docs.rs/rand/latest/rand/distributions/struct.Standard.html

多くのプリミティブ型に対してDistributionが実装されている汎用的な分布。

基本的には、数値的に一様な分布を持ち、型に適した範囲の値を生成する。

- Integer（i32, u32など）: すべての値に対する一様分布
- char: すべてのUnicodeの値に対する一様分布。つまり、0 ～ 0x10_FFFF（ただし 0xD800 ～ 0xDFFF を除く）の範囲
- bool: falseかtrueをそれぞれ0.5の確率で生成
- Float（f32, f64）: `[0, 1)`の範囲の一様分布
- `Option<T>`:まずbool値を生成し、trueならT型の値valueを生成してSome(value)を返し、falseならNoneを返す
- Tuple（最大12要素）: 各要素ごとに生成
- Array: （最大32要素）
  - 各要素を順次生成
  - 要素の型が小さい場合は、Rng::fillを使った方が最適化されており早い傾向がある
  - 32要素より多くの要素の配列を作りたい場合は、randの`min_const_gen`featureを加えることで作成できるようになる
- String: DistStringトレイトが実装されている。内部的には`Distribution::<char>::sample_iter`が使用されている

また、基本的にはRngCoreトレイトのメソッドを使用して実装されているので、RngCoreが一様分布にそった乱数を生成することを期待して実装されている。

Standard構造体は、Rngトレイトのgenの内部で使用されている構造体でもあるので、Rngトレイトのgenメソッドの出力も上記のようになっている。

#### Open01, OpenClosed01構造体

https://docs.rs/rand/latest/rand/distributions/struct.Open01.html

Open01構造体は浮動小数点数を開区間`(0, 1)`で一様にサンプリングする分布である。

https://docs.rs/rand/latest/rand/distributions/struct.OpenClosed01.html

一方、OpenClosed01構造体は半開区間`(0, 1]`で一様にサンプリングする分布である。

Standardの場合は、浮動小数点数を開区間`[0, 1)`で一様にサンプリングするのが、これら3つが若干異なっているが、それ以外については同様の実装である。

#### Uniform構造体

https://docs.rs/rand/latest/rand/distributions/uniform/index.html

Uniform構造体は`distributions::uniform`モジュールで実装されており、Uniform以外にもいくつかの構造体やtraitが実装されているが、階層がだいぶ深くなってきたので、ここにまとめて書く。

uniformモジュールは与えられた範囲内の数値を一様にサンプリングする分布についてのモジュールである。

https://docs.rs/rand/latest/rand/distributions/uniform/struct.Uniform.html

単純に範囲を指定して一様分布な乱数を得たい場合、基本的に以下の2つの選択肢になる。

- Rng::gen_rangeを使用する
- Rng::sample（もしくはsample_iter）にDistributionを実装しているUniformを渡して使用する（もしくはUniformにRngを渡す）

gen_rangeは`gen_range<T, R>(&mut self, range: R) -> T where T: SampleUniform, R: SampleRange<T>`であったが（Rng構造体についてを参照）、rangeには基本的な型に対する`Range<T>`（例えば0..10など）であれば問題なく使用できるため、あまりuniformモジュールを意識する必要はない。

同様にRng::sampleの方も、Uniformのインスタンス化ができれば、あまりuniformモジュールについては意識する必要はない。

意識するとしたら、自分で用意した型に対して、gen_rangeやsampleを使用したいときなどであろう。

そのため、ここではまずUniform構造体について書き、そのあとで各traitや構造体について書いていく。

https://docs.rs/rand/latest/rand/distributions/uniform/struct.Uniform.html

Uniform構造体は、様々な型に対するDistributionを実装している、範囲指定の一様分布である。

以下の2つのインスタンス化メソッドがある。

- new: new(low, high)で、`[low, high)`のUniformを作成
- new_inclusive: new_inclusive(low, high)で、`[low, high]`のUniformを作成

また`From<Range<X>>`トレイトを実装しているので、`Uniform::from(10..10000)`などのようにインスタンス化することもできる。

https://docs.rs/rand/latest/rand/distributions/uniform/index.html

自分で作成した型Xに対して、gen_rangeやsampleを使用したいときは、以下が必要になる。

- UniformSamplerトレイトを実装した型Y
  - ここに型Xを範囲指定で一様に取り出す方法を実装する
- YをSampler typeとして指定してSampleUniformトレイトを型Xに実装

このようにすると、Uniformでこの型Xを使用することができるようになる。

また、これらを実装することで、SampleRangeトレイト（Rng::gen_rangeに必要なトレイト）もそのまま実装される。

以下ではこれらについてまとめていく。

https://docs.rs/rand/latest/rand/distributions/uniform/trait.SampleUniform.html

SampleUniformトレイトは、指定した範囲（0..10などのRangeやlow, highなどの指定）から、適切なサンプラーを取得するためのトレイトである。

このトレイトを実装しているおかげで、指定した範囲の型情報から、適切な「UniformSamplerトレイトを実装した型」を見つけることができる。

SampleUniformトレイトには以下を定義する必要がある。

- Sampler type: UniformSamplerトレイトを実装している型

https://docs.rs/rand/latest/rand/distributions/uniform/trait.UniformSampler.html

UniformSamplerトレイトは、範囲からどのように一様に値を取得するかを実装するためのトレイトである。

uniformモジュールでは、以下のそれぞれの型に対して、UniformSamplerトレイトを実装した構造体があり、それをUniform内で呼び出すことで、基本的な型に対してはUniformを使用することができるようになっている。

- char: UniformChar
- time::Duration: UniformDuration
- Integer: UniformInt
- Float: UniformFloat

自前の型にUniformSamplerトレイトを実装する場合は、以下を実装する必要がある。

- type: この実装でサンプリングする型
- new: low, highを指定して、`[low, high)`のサンプラーをインスタンス化（Uniform::newの内部実装）
- new_inclusive: low, highを指定して、`[low, high]`のサンプラーをインスタンス化（Uniform::new_inclusiveの内部実装）
- sample: 範囲から一様に値をサンプリングする

また、以下のメソッドが提供されている。

- sample_single
  - 引数にlow, high, rngを指定して、newとsampleをいっぺんに行うメソッド
  - デフォルトではUniformSampler::new(low, high).sample(rng)が行われている
  - 一つだけ取得する場合により効率的な実装がある場合は、このメソッドを実装するといい
- sample_single_inclusive: sample_singleのnew_inclusiveとsampleをいっぺんに行うバージョン

https://docs.rs/rand/latest/rand/distributions/uniform/trait.SampleBorrow.html

UniformSamplerトレイトの各メソッドの引数low, highはSampleBorrowトレイトが実装されている必要があるが、これはSampleUniformトレイトを実装すると自然に実装される。

https://docs.rs/rand/latest/rand/distributions/uniform/trait.SampleRange.html

SampleRangeトレイトは、SampleBorrowトレイトが実装されている型のRangeとRangeInclusiveには自然に実装される。

以下のメソッドが定義されている。

- sample_single: デフォルトではUniformSampler::sample_singleが使用されている
- is_empty: 範囲内に要素がないならtrueを返す（例えば0..2には1が含まれるが、0..1には要素がないのでis_emptyはtrueを返す）

#### Bernoulli構造体

https://docs.rs/rand/latest/rand/distributions/struct.Bernoulli.html

bool型に対してDistributionが実装されている、ベルヌーイ分布の構造体。

trueになる確率を内部で保持し、その確率にあわせてtrue,falseを得ることができる。

以下2つのメソッドが実装されている。

- new: 確率（f64）でインスタンス化する
- from_ratio: numerator（u32）とdenominator（u32）を指定して確率 numerator / denominatorでインスタンス化する

Rngのgen_boolとgen_ratioはこれらのメソッドを内部で呼び出している。

#### Alphanumeric構造体

https://docs.rs/rand/latest/rand/distributions/struct.Alphanumeric.html

ASCII文字と数字（a-z、A-Z、0-9）に一様に分散したu8をサンプリングする構造体。

u8に対するDistributionと、DistStringを実装している。

#### Slice構造体

https://docs.rs/rand/latest/rand/distributions/struct.Slice.html

スライスから一様にサンプリングする構造体である。

任意のスライスに対してDistributionが実装されている。

例えば`x=['a', 'b', 'c']`に対して `Slice::new(&x)`とすれば、a,b,cの中からランダムに値を取得できる。

一度だけ取得したい場合は、SliceRandom::chooseの方が効率的である。（SliceRandomについては後述）  
また、Sliceは重複を許すため、重複を避けたい場合は`SliceRandom::choose_multiple`を使用した方がいいかもしれない。（SliceRandomについては後述）

Sliceには以下のメソッドが実装されている。

- new: スライスを渡してインスタンス化する

#### WeightedIndex構造体

https://docs.rs/rand/latest/rand/distributions/struct.WeightedIndex.html

離散項目の重み付きサンプリングを使った分布である。

usizeに対するDistributionを実装している。

初期化時に`[2, 1, 1]`のような配列などを渡し、渡した配列のindexに対応するusizeの値をランダムに返す。  
ただし、indexが選ばれる確率は、渡した配列の要素の値に比例する。  
例えば`[2, 1, 1]`であれば、0を返す確率が1/2で、1,2を返す確率がそれぞれ1/4である。

重みには`Uniform<X>`の実装が存在する任意の型 X を使用することができる。

WeightedIndexには以下のメソッドが定義されている。

- new: イテレータを渡して初期化する
- update_weights
  - new_weights（`&[(usize, &X)]`）を渡して、重みを更新する
  - new_weightsのusizeの部分はindexを表し、&Xの部分は更新する値を表す
  - new_weightsはindexについてソートされている必要がある

なお、内部でallocateする可能性があるため、feature allocを有効にする必要がある。  
（デフォルトの状態だと有効になっている）

また、重みの型に対して`Uniform<X>`が実装されている必要がある理由については、内部の実装で、`Uniform<X>`を使用しているからである。

具体的な実相内容としては、重みの配列をすべて足し合わせて、0からその和までに対して、Uniformで値を取得し、その値がどのindexの位置にあるかを確認することで、indexの値を返しているようである。

そのため、WeightedIndexからのサンプリングにかかる時間はO(log N)のようである。  
別の方法として、rand_distr::weighted_aliasはO(1)のサンプリングをサポートするが、初期化コストが非常に高くなるようである。  
（rand_distについてはここではまとめない）

### seqモジュール

https://docs.rs/rand/latest/rand/seq/index.html

シーケンス関連の機能が実装されているモジュール。

主に以下の機能が提供されている。

- SliceRandomトレイト: スライスに対する拡張トレイト
- IteratorRandomトレイト: イテレータに対する拡張トレイト
- index::sample関数: 0～lengthの値を取得することに特化した関数

#### SliceRandomトレイト

https://docs.rs/rand/latest/rand/seq/trait.SliceRandom.html

スライスに対する拡張トレイトで、スライス`[T]`に対しては自動で実装される。

要素を選択したり、シャッフルしたりなどのメソッドが提供されている。

自分で用意した型にSliceRandomトレイトを実装する場合は、定義されている以下の要素をすべて実装する必要がある。

- Item type: 要素の型
- choose: スライスのランダムな要素への参照を返す。スライスが空ならNoneを返す
- choose_mut: スライスのランダムな要素への変更可能な参照を返す。スライスが空ならNoneを返す
- choose_multiple
  - feature allocを有効時のみ利用可（デフォルトは有効）
  - スライスからランダムに、繰り返しなしで、ランダムな順序で要素を選択する
  - SliceChooseIterを返すが、これはIteratorトレイトが実装されているので、イテレータとして扱えばいい
  - より柔軟な方法としてseq::index::sampleがある（後述）
- choose_weighted
  - feature allocを有効時のみ利用可（デフォルトは有効）
  - chooseに似ているが、各結果の尤度を指定することができる
  - 関数weightを指定して、各項目xを相対尤度weight(x)にマップし、その尤度に比例する確率で選択される
  - 内部的にはdistribution::WeightedIndexを使用している
- choose_weighted_mut: choose_weightedの変更可能な参照を返すバージョン
- choose_multiple_weighted: choose_weightedとchoose_multipleをあわせたようなメソッド
- shuffle: 要素をシャッフルする
- partial_shuffle
  - シャッフルする要素数amountを指定してシャッフルする
  - 出力は、シャッフルした要素の配列（lenはamount）とシャッフルされていない要素の配列（元の配列からシャッフルした要素が取り除かれているもの）

#### IteratorRandomトレイト

https://docs.rs/rand/latest/rand/seq/trait.IteratorRandom.html

イテレータに対する拡張トレイトで、イテレータ（Iterator + Sized）に対しては自動で実装される。

ランダムに値を取得するためのメソッドが提供されている。

以下のメソッドが提供されている。

- choose
  - イテレータからランダムに要素を一つ選択する（空の場合はNoneを返す）
  - このメソッドは最適化のために Iterator::size_hint を使用する
    - 正確なヒントがあり、Iterator::nth が定数時間演算である場合はO(1)
    - サイズのヒントがない場合はO(n)
  - 同じイテレータでも異なる結果になることがある（非安定的）
- choose_stable
  - ほとんどchooseと同じだが、安定的
  - どのようなイテレータを扱っても一貫した結果が必要な場合に有用
- choose_multiple_fill
  - 引数に渡したバッファにランダムに値を埋めていく
  - 要素はランダムに選択されるが、順番はランダムではない
  - バッファに追加された要素の数を返すが、イテレータに十分な要素が含まれてれば、基本的にはバッファの長さと等しくなる
  - スライスの場合はSliceRandom::choose_multipleを使う方がよい
- choose_multiple
  - feature allocを有効時のみ利用可（デフォルトは有効）
  - 要素数を指定して、その要素の長さのベクトルを作成する
  - ベクトルを作成する点以外はchoose_multiple_fillと同様である

#### indexモジュール

https://docs.rs/rand/latest/rand/seq/index/index.html

使用するにはfeature allocを有効にする必要がある（デフォルトは有効）。

indexモジュールはインデックスをサンプリングするためのより低レベルなAPIである。

indexを取得する、つまり、usizeで0～lengthの値をランダムに取得するという点においてより最適化さている。

基本的には以下２つの関数を使用することになる。

- index::sample: 0～lengthでamount個ランダムにサンプリング
- index::sample_weighted: 重み付きでサンプリング

https://docs.rs/rand/latest/rand/seq/index/fn.sample.html

sample関数は以下のようになっている。

```rust
pub fn sample<R>(rng: &mut R, length: usize, amount: usize) -> IndexVec 
where
    R: Rng + ?Sized, 
```

0からlengthの範囲から、amount個ランダムにサンプリングする。（順序もランダム）

IndexVecは最適化のために使用されているVecのような型で、以下が実装されている。

- len: 要素数
- is_empty: 空かどうか
- index: 引数にわたすindex番目の値を返す
- into_vec: `Vec<usize>`への変換
- iter: IndexVecIter（イテレータ）への変換

https://docs.rs/rand/latest/rand/seq/index/fn.sample_weighted.html

sample_weighted関数は以下のようになっている。

```rust
pub fn sample_weighted<R, F, X>(
    rng: &mut R, 
    length: usize, 
    weight: F, 
    amount: usize
) -> Result<IndexVec, WeightedError> 
where
    R: Rng + ?Sized,
    F: Fn(usize) -> X,
    X: Into<f64>, 
```

この関数の使用にはfeature stdも有効にする必要がある（デフォルトで有効）。

sample関数と異なる点としては、重みを関数weightによって与えることができる。  
この重み関数weightは各インデックスに対して1回だけ呼び出される。

また、順序に関しての保証（シャッフルされるかなど）はない。

### rngsモジュール

https://docs.rs/rand/latest/rand/rngs/index.html

RngCoreを実装した基本的な構造体が定義されている。

- StdRng: もっとも標準的なPRNGで、安全性も担保されている
- adapterモジュール: シードを定期的にリセットするReseedingRngが定義されている
- ThreadRng: シードを定期的にリセットするStdRng
- SmallRng: 小さくて高速な安全でないPRNG
- mockモジュール: テスト用に使用するStepRngが実装されている（乱数を生成するわけではないがテストのためにRngCoreが実装されている）

#### StdRng構造体

https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html

標準的なRng。

StdRngには、現在のプラットフォームで効率的で、統計的に強く、予測不可能（暗号的に安全）なものが選択されている。

現在は[ChaCha12Rng](https://docs.rs/rand_chacha/latest/rand_chacha/struct.ChaCha12Rng.html)が使用されており、StdRngはChaCha12Rngの薄いラッパーになっている。

このアルゴリズムは決定論的だが、将来のライブラリバージョンで別のアルゴリズムに置き換えられる可能性があるため、再現可能とはみなされない。

再現性を考えるのであれば、ChaCha12Rngを直接使用することが推奨されている。

StdRngにはRngCore（つまりRng）、SeedableRng、CryptoRngが実装されている。

また、feature std_rngが有効である必要がある（デフォルトでは有効）

#### adapterモジュール

https://docs.rs/rand/latest/rand/rngs/adapter/index.html

Rngに関するラッパーやアダプターが実装されているモジュール。

feature stdが有効である必要がある（デフォルトでは有効）。

ReadRngはrandのversion0.8.4から削除されているので、実質ReseedingRngのみ実装されている。

https://docs.rs/rand/latest/rand/rngs/adapter/struct.ReseedingRng.html

ReseedingRngはBlockRngCoreを実装したPRNGのラッパーで、シードをリセットする機能を持っている。

以下のタイミングでシードをリセットする。

- 手動でreseed()を呼び出したとき
- clone()を行ったとき
- UNIXでプロセスがフォークされると、親プロセスと子プロセスの両方で
- インスタンス化の際に設定できる数の生成バイト数を超えたとき

このようにシードを再接種することは、安全面において必ずしも必要ではないようだが、ある種の「深層におけるセキュリティ」と見なすことができると書いてあった。

将来、使用されているCSPRNGに暗号的な弱点が見つかったり、実装に欠陥が見つかったりしても、時折シードをとりかえることで、それを悪用することがより困難になったり、不可能になったりするはずとのこと。

ReseedingRngには以下のメソッドが実装されている。

- new
  - rng（BlockRngCore + SeedableRngが必要）とthreshold（シードをとりなおす生成バイト数）とreseeder（RngCoreが必要。シードを生成する用のRNG）が引数
  - thresholdは0だと定期的なシードのリセットを行わなくなる（上の4つめのリセット方法がなくなる）
- resees: シードをリセットする

#### ThreadRng構造体

https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html

StdRngと同じPRNGで、64kiB毎にシートがOsRngによってリセットされるPRNG。

内部的にはReseedingRngのrngとしてStdRngと同じPRNGを、reseeder（シードのリセット用のRng）にOsRngを使用したものになっている。  
また、thresholdは1024 * 64に、つまり64 kiBに設定されている。

シードは適宜リセットされるため、SeedableRngは実装されておらず、RngCoreとCryptoRngが実装されている。

#### SmallRng

https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html

小さくて高速な非暗号PRNG。

安全ではないため、そういった用途では推奨されない。

RngCoreとSeedableRngは実装されているが、CryptoRngは実装されていない。

現在は64ビット環境では[rand_xoshiro::Xoshiro256PlusPlus](https://docs.rs/rand_xoshiro/0.6.0/rand_xoshiro/struct.Xoshiro256PlusPlus.html)、32ビット環境では[rand_xoshiro::Xoshiro128PlusPlus](https://docs.rs/rand_xoshiro/0.6.0/rand_xoshiro/struct.Xoshiro128PlusPlus.html)が使用されている。

#### mockモジュール

https://docs.rs/rand/latest/rand/rngs/mock/index.html

テストなどで使用する、RngCoreが実装されているものの、実際は乱数を生成するものではないものが実装されているモジュール。

現在はStepRngのみが実装されている。

https://docs.rs/rand/latest/rand/rngs/mock/struct.StepRng.html

StepRngは、initialからincrement分ずつ可算した値を返すテスト用のmockである。

newメソッドが定義されていて、initialとincrementを引数にとる。

例えば`rng = StepRng::new(2, 3)`としたら、rngは2,5,8と順に値を生成する。

StepRngにはRngCoreが実装されているが、もちろん乱数を生成する目的で使用することは想定されていない。

例えばRngCoreを受け取る関数などのテストを作成したいときに、このStepRngを使用すると出力される結果がわかるためテストが作成しやすい。

### thread_rng関数

https://docs.rs/rand/latest/rand/fn.thread_rng.html

ThreadRngを作成する関数。

詳しくはThreadRng構造体を参照

### random関数

https://docs.rs/rand/latest/rand/fn.random.html

乱数値を一つ生成する。

これはthread_rng().gen() のショートカットである。

## 参考

- The Rust Rand Book: https://rust-random.github.io/book/intro.html
- 各クレートのドキュメント
  - https://docs.rs/getrandom/latest/getrandom/index.html
  - https://docs.rs/rand_core/latest/rand_core/index.html
  - https://docs.rs/rand/latest/rand/index.html
- github: https://github.com/rust-random/rand
- Linuxの乱数ジェネレータについて: https://utakamo.com/article/linux/architecture/directory/dev/random-urandom.html
