# cargo-zigbuild

https://github.com/rust-cross/cargo-zigbuild

## memo

### cargo-zigbuildとは

リンカとしてzigを使用してクロスコンパイルを行うツール

### install

https://github.com/rust-cross/cargo-zigbuild?tab=readme-ov-file#installation

https://github.com/rust-cross/cargo-zigbuild?tab=readme-ov-file#usage

以下でinstall

```sh
$ cargo install cargo-zigbuild
```

また、zig自体のinstallも必要で、色々な方法があるようだが（上のリンク参照）、以下でinstallすることができる

```sh
$ pip3 install ziglang
```

### 使用方法

https://github.com/rust-cross/cargo-zigbuild?tab=readme-ov-file#usage

targetを指定して以下のようにすることでビルドできる

```sh
cargo zigbuild --target aarch64-unknown-linux-gnu
```

targetのlistは以下で取得できる

```sh
$ rustc --print target-list
```

また以下のようにしてversionを指定することも可能

```sh
cargo zigbuild --target=x86_64-unknown-linux-gnu.2.17
```

### crossとの違い

crossではDockerコンテナなどを介してクロスコンパイルを行う。

それに対して、cargo-zigbuildではコンテナなどを介さずに、Zigを使用してクロスコンパイルを行う。

そのため、cargo-zigbuildは依存するものが少なく、また軽量である。

一方、cargo-zigbuildはリンカやコンパイラの指定を`--target`にあわせて行ってはくれるものの、それ以上のことは（簡単には）できない。

例えば、WindowsAPIを使用する場合、crossでは`cross build --target x86_64-pc-windows-gnu`とすればよいが、`cargo zigbuild --target=x86_64-pc-windows-gnu`としてもビルドすることができない。

また、cargo-zigbuildはビルドを行うことはできるが、実行やテストを行うことはできない。

### 参考

- zig ccについて：https://andrewkelley.me/post/zig-cc-powerful-drop-in-replacement-gcc-clang.html
- 使用例1：https://naari.hatenablog.com/entry/2022/08/12/091404
- 使用例2：https://zenn.dev/skanehira/articles/2022-07-12-rust-rjo
- 使用例3：https://qiita.com/twrcd1227/items/a45eb0ba4853c5b41d8b#%E3%82%AF%E3%83%AD%E3%82%B9%E3%82%B3%E3%83%B3%E3%83%91%E3%82%A4%E3%83%A9
- Rustでバイナリを配布する場合のそれぞれの方法の利点や、cargo-zigbuildの強みについて：https://zenn.dev/coord_e/articles/portable-binary-in-rust
