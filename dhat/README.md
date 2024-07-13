# dhat

https://docs.rs/dhat/latest/dhat/

## dhatとは

ヒープ解析ができるクレート

## 設定

以下で依存関係を追加。

```dhat
$ cargo add dhat
```

また、加えてCargo.tomlに以下の設定をすることが推奨されている。

```
[profile.release]
debug = 1
```

https://docs.rs/dhat/latest/dhat/#configuration-profiling-and-testing

## heap profiling

dhatでは２つのモードがあり、こちらのheap profilingでは、システムアロケータをラップするグローバルアロケータを使用して、すべてのヒープ割り当てを追跡し、解析を行うことができる。

### 設定

ここでは、[フィーチャーフラグ](https://qiita.com/osanshouo/items/43271813b5d62e89d598)を設定して、以下のようにしたときのみ、ヒープ解析ができるようにする。

```sh
cargo run --release  --features dhat-heap
```

※ releaseはあってもなくてもいいが、dhatを使用すると遅くなるため、使うことを推奨されている  
※ 常にdhatを使用するのは遅く、また解析時のみ設定するのもめんどうなので、フィーチャーフラグを使用するのが推奨されているのだと思われる

まず、Cargo.tomlに以下を加える。

```toml
[features]
dhat-heap = []
```

次に、以下をプログラムに追加する。

```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
```

また、main関数の最初に以下を追加。

```rust
#[cfg(feature = "dhat-heap")]
let _profiler = dhat::Profiler::new_heap();
```

これで`cargo run --release --features dhat-heap`で解析ができるようになる。

### 結果の見方

結果は以下のように表示される。

```
dhat: Total:     1,063 bytes in 4 blocks
dhat: At t-gmax: 1,050 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
```

なお、「block」は「allocation」と同義語として使われているようである。

- Total: 実行全体でどれだけのヒープブロックとバイトが割り当てられたか
- At t-gmax: （バイト単位の）ヒープサイズが最大の時点でのヒープブロックとバイト
- At t-end: 実行終了時のヒープブロックとバイト（これはつまり、明示的に解放されなかったもの）

また、実行時に`dhat-heap.json`が作成されるが、ここにはヒープされた場所などが書かれたバックトレースが格納されている。

これを見るためには、online viewer を使用するか、Valgrindのレポジトリをクローンして確認する方法がある。

online viewerのリンクは以下。

https://nnethercote.github.io/dh_view/dh_view.html

また、クローンする方法やバックトレースなどについての詳細は以下。

https://docs.rs/dhat/latest/dhat/#viewing

### テスト

https://docs.rs/dhat/latest/dhat/#heap-usage-testing

例えばヒープ割り当てが意図通りに行われているか、終了時点で正しく開放できているかなどのテストを行うことができる。

ただし、複数のprofilerを使用するとパニックになるため、デフォルトで並列に実行されるテストとは相性が悪い。

そのため、統合テストに一つだけテスト関数を作成する方法を推奨している。  
この方法であれば、統合テストは各ファイルごとに独自のプロセスで実施され、他と干渉することがなく、またテスト関数が一つであれば、並列に実行されることもないからである。

これが難しい場合、`cargo tests -- --test-threads=1`とすれば、並列には実行されなくなるが、すべてのテストが並列に実行されないため、テストに時間がかかるというデメリットがある。

その他の方法や詳細は、上記リンクを参照。

統合テストの例として、以下があげられていた。

```rust
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn test() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let _v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    drop(v2);
    let v3 = vec![9, 10, 11, 12];
    drop(v3);

    let stats = dhat::HeapStats::get();

    // Three allocations were done in total.
    dhat::assert_eq!(stats.total_blocks, 3);
    dhat::assert_eq!(stats.total_bytes, 48);

    // At the point of peak heap size, two allocations totalling 32 bytes existed.
    dhat::assert_eq!(stats.max_blocks, 2);
    dhat::assert_eq!(stats.max_bytes, 32);

    // Now a single allocation remains alive.
    dhat::assert_eq!(stats.curr_blocks, 1);
    dhat::assert_eq!(stats.curr_bytes, 16);
}
```

### profilerがdropされない場合

main関数で追加した`_profiler`がDropされるタイミングで解析と出力の行われるため、例えば `std::process::exit`などを呼び出すとDropされず、うまく解析ができない場合があるようである。  
その場合は、終了の直前に`_profiler`を明示的にDropする必要がある。

## ad hoc profiling

dhatのもう一つのモードであるad hoc profilingは、`dhat::ad_hoc_event`を差し込んだ箇所が何回呼ばれているかを確認することができる。

基本的には`heap profiling`の場合と同じように使用できるため、違いのみ書いていく。

### 設定

`heap profiling`の場合と同様に、フィーチャーフラグを設定して、以下のようにしたときのみ、ヒープ解析ができるようにする。

```sh
cargo run --release --features dhat-ad-hoc
```

まず、Cargo.tomlに以下を加える。

```toml
[features]
dhat-ad-hoc = []
```

次に、main関数の最初に以下を追加。

```rust
#[cfg(feature = "dhat-ad-hoc")]
let _profiler = dhat::Profiler::new_ad_hoc();
```

あとは調べたい箇所に以下を追加する。（複数追加してもよい）

```rust
#[cfg(feature = "dhat-ad-hoc")]
dhat::ad_hoc_event(1);
```

ad_hoc_eventの引数`weight`については後述。（基本的には1でいい）

これで`cargo run --release  --features dhat-ad-hoc`で解析ができるようになる。

### その他

結果は以下のように表示される。

```
dhat: Total:     1,000 units in 10 events
```

- events: `dhat::ad_hoc_event`が呼ばれた回数
- units: ad_hoc_eventの引数`weight`を考慮した値
  - 例えば、`dhat::ad_hoc_event(100)`が10回呼ばれると1,000 unitsになる
  - 例えば、`dhat::ad_hoc_event(100)`が10回、`dhat::ad_hoc_event(4)`が5回呼ばれると、1,020 unitsになる

`weight`引数はunitsに影響を与える。  
例えば、重い処理は`weight`を大きくするなどの使い方ができるかもしれない。

`heap profiling`の場合と同様に、`dhat-ad-hoc.json`が出力されるため、viewerを使用して、バックトレースを確認することができる。

また、テストについても同様に行うことができるが、統合テストの例としては以下のようになる。

```rust
fn ad_hoc_test(weight: usize) {
    dhat::ad_hoc_event(weight);
}

#[test]
fn test() {
    let _profiler = dhat::Profiler::builder().ad_hoc().testing().build();

    for _ in 0..10 {
        ad_hoc_test(100);
    }

    for _ in 0..5 {
        ad_hoc_test(4)
    }

    let stats = dhat::AdHocStats::get();

    dhat::assert_eq!(stats.total_events, 15);
    dhat::assert_eq!(stats.total_units, 1020);
}
```
