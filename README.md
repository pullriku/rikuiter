# rikuiter


Rustの `Iterator` トレイトを理解するために、**ゼロからIteratorを自作してみる**学習用プロジェクトです。

## 📦 実行方法

各ブランチにチェックアウトして、`cargo`で実行できます。

```bash
git checkout playground
cargo run
```

## Examples

### Collatz 数列

`collatz`はコラッツ数列を返すイテレータのテストです。ついでにGraphvizを使って可視化しています。
`.dot`ファイルを出力するため、`dot`コマンドがあれば可視化できます。

```sh
cargo run --release --example collatz
dot -Tpdf graph.dot -o graph.pdf
```
