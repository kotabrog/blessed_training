# cross

https://github.com/cross-rs/cross

## memo

### crossとは

クロスコンパイルを行うためのツール

### クロスコンパイルとは

クロスコンパイルは、コンパイラが実行されているプラットフォーム以外のプラットフォーム向けの実行可能コードを作成するプロセス

プラットフォームの違いとは、OSが異なることやアーキテクチャが異なること

https://learn.microsoft.com/ja-jp/dotnet/core/deploying/native-aot/cross-compile

### 動かし方の例

WSL上で開発を行いWindows上でそれを実行する例

dockerを起動しておく必要がある

私の環境だと以下を実行

```sh
$ sudo service docker start
```

そのあとは以下の記事の通りに

https://blog.foresta.me/posts/rust-cross-build-for-windows-on-wsl/

基本的にはcargoと同じCLIを持っており、`cargo build`などのかわりに`cross build`とすればよいらしい

https://github.com/cross-rs/cross?tab=readme-ov-file#usage

### 依存

https://github.com/cross-rs/cross?tab=readme-ov-file#dependencies

通常はrustupを使用していると思うのでinstall方法については心配はいらない

rustup以外を使用している場合は上記を参照

またdefaultだとdockerを使用してクロスコンパイルを実現している

docker以外を使用する場合も上記を参照

また、dockerを使用する場合は、権限の問題が発生する可能性があるため注意

### 設定

https://github.com/cross-rs/cross?tab=readme-ov-file#configuration

https://github.com/cross-rs/cross/wiki/Configuration

https://github.com/cross-rs/cross/blob/main/docs/config_file.md

色々な設定方法・内容があるため上記を参照

### dockerの中でクロスコンパイルを行う

https://github.com/cross-rs/cross?tab=readme-ov-file#docker-in-docker

コンテナ内でクロスコンパイルを実行する場合は、ホストのdockerデーモン自体にcrossがアクセスできるようにする必要がある

詳細は上記を参照

### テスト時の注意

https://github.com/cross-rs/cross?tab=readme-ov-file#supported-targets

こちらに書いてある通りであれば、`cross test`ではスレッドは1つで実行されるようである

テストやそのほか`run`時にはコンテナ内でQEMUを使用しているようで（必ず使用しているわけではないかもしれない。CPUが異なるときだけ？：https://qiita.com/termoshtt/items/12f9dbb5425a94a4e545#qemu ）、その関係でスレッドを複数作成するとうまくいかないことがあり、このようになっているようである

### 対応しているターゲット

https://github.com/cross-rs/cross?tab=readme-ov-file#supported-targets


### 参考

https://github.com/cross-rs/cross/wiki

詳しい内容についてはcrossのwikiが参考になりそう

CIについてなどものっている

### WSL2でWindows環境のテストをするときの注意（未解決）


WSL2でwindows環境のテストをする際、通常であれば以下のようにすることで実行ができる

```sh
$ cross test --target x86_64-pc-windows-gnu
```

しかし以下のエラーが発生

https://github.com/cross-rs/cross/issues/1277

こちらに書いてある通りDocker Desktopでの設定で問題が解決する場合もあるようだが、私の環境では解決しなかった
