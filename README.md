# 二元情報源を2次・3次に拡大する学習

大学のある科目の課題として取り組んだものです。二元情報源を2次、3次へと拡大することでエントロピーやハフマン木がどのように変化するかを観測しています。

Rust実行環境は入っているものとし、本リポジトリに移動後次のコマンドで結果を見ることができます。

コマンドライン引数の第一引数は次元、第二引数は `0` の生起確率です。指定子ない場合、次元は `1` 、確率は `0.5` になります。

```bash
$ cargo run 2 0.4
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/nd_binary 2 0.4`
entropy: 1.9419011889093372
. (1.00)
├── 0 (0.40)
|   ├── 00 (0.16) 00
|   └── 01 (0.24) 01
└── 1 (0.60)
    ├── 10 (0.24) 10
    └── 11 (0.36) 11

00 (0.16000000000000003) => 00
01 (0.24) => 01
10 (0.24) => 10
11 (0.36) => 11
avg_len: 2
```

なお、ライブラリとして https://github.com/o1810156/huffman を使用しています。

# 使用したバージョン

```bash
$ cargo --version
cargo 1.40.0 (bc8e4c8be 2019-11-22)
$ rustc --version
rustc 1.40.0 (73528e339 2019-12-16)
```
