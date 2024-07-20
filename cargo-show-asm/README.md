# cargo-show-asm

https://docs.rs/cargo-show-asm/latest/cargo_show_asm/

## cargo-show-asmとは

Rustコードに対して生成されたAssembly、LLVM-IR、MIR、WASMを表示するツール

## 使用方法

例えば、`cargo new ...`をしたばかりで、main()に`println!("Hello, world!");`と書いているだけのプロジェクト内で、以下のように実行する。

```sh
$ cargo asm
```

すると、私の環境（WSL2）だと以下のように表示される。

```sh
Try one of those by name or a sequence number
0 "core::ops::function::FnOnce::call_once{{vtable.shim}}" [20]
1 "core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>" [7]
2 "main" [18]
3 "std::rt::lang_start" [28]
4 "std::rt::lang_start::{{closure}}" [19]
5 "std::sys_common::backtrace::__rust_begin_short_backtrace" [22]
6 "test_show_asm::main" [28]
```

この中だと、`6 "test_show_asm::main" [28]`がmain関数を指すので、例えば以下のように実行すると、main関数のアセンブリが表示される。

```sh
$ cargo asm 6
```

また、以下のようにしてもよい。

```sh
$ cargo asm test_show_asm::main
```

もしくは、以下のようにしても、一つに絞れるため、結果が表示される。

```sh
$ cargo asm test_show_asm::m
```

例えば以下のようにした場合は一つに絞れないため、再び選択肢が表示される。

```sh
$ cargo asm --bin test_show_asm main

...

Try one of those by name or a sequence number
0 "main" [18]
1 "test_show_asm::main" [28]
```

このように、番号を指定するか、もしくは関数名を指定して部分一致で一つに絞ることができれば、実行される。

## 対象の選択

大きなカテゴリとして以下を指定できる。

- `--lib`: `src/lib.rs`
- `--test Test`: `tests/Test`
- `--bench BENCH`: `benches/BENCH`
- `--example EXAMPLE`: `examples/EXAMPLE`
- `--bin BIN`: `src/bin/BIN`もしくは`Bin/src/main.rs`

例えば、上記の「使用方法」の例の場合は、以下のようにしてもよい。（プロジェクト名は`test_show_asm`）

```sh
$ cargo asm --bin test_show_asm main
```

上記の「使用方法」の例ではまだ`src/main.rs`しかなかったため、このカテゴリの指定は必要なかったが、例えば`src/lib.rs`を作成した状態だと以下のようになる。

```sh
$ cargo asm

test_show_asm defines multiple targets, you need to specify which one to use:
        --lib
        --bin test_show_asm
Error: Multiple targets found
```

そして `--bin test_show_asm` を加えると、「使用方法」の例と同様に、どの関数にするかの選択肢が出力される。

```sh
$ cargo asm --bin test_show_asm

...

Try one of those by name or a sequence number
0 "core::ops::function::FnOnce::call_once{{vtable.shim}}" [20]
1 "core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>" [7]
2 "main" [18]
3 "std::rt::lang_start" [28]
4 "std::rt::lang_start::{{closure}}" [19]
5 "std::sys_common::backtrace::__rust_begin_short_backtrace" [22]
6 "test_show_asm::main" [28]
```

## 出力形式の指定

出力形式は以下が選択可能。  
一部、Cargo.tomlにfeatureの指定をする必要があるものもある。（実行時に注意がでる）

- --asm: Assembly（default）
- --disasm: Disassembly binaries or object files
- --llvm: llvm-ir
- --llvm-input: llvm-ir before any LLVM passes
- --mir: MIR
- --wasm: WASM, needs wasm32-unknown-unknown target installed
- --mca: llvm-mca anasysis
- --intel: Intel style for assembly
- --att: AT&T style for assembly

## 関数がないときは

以下の場合、関数を選択することができない。

- publicになっていない
- inline化されている
- ジェネリックパラメータをもつ

インライン化されている場合は `#[inline(never)]` をつけることで解決できる。

ジェネリックパラメータを持つ場合は、その関数をラップするジェネリックパラメータを持たない関数を作成する必要がある。  
例えば以下のようにする。

```rust
pub fn generic<T>(x: T) -> T {
    x
}

#[inline(never)]
pub fn generic_wrapper(x: i32) -> i32 {
    generic(x)
}
```

※ 上の`generic_wrapper()`はinline化されてしまうため、`#[inline(never)]`をつける必要がある
