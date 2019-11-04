# rust-async-await-sample

[ついに正式対応されるRustのawait/asyncを1\.39betaで確認してみた \- Qiita](https://qiita.com/SoraKumo/items/54a11d5a70cd787d496c)の写経。

## 環境

```
❯ rustup --version
rustup 1.20.2 (13979c968 2019-10-16)

❯ cargo version
cargo 1.40.0-nightly (5da4b4d47 2019-10-28)
```

## 実行結果

```
❯ cargo run
foo1:0 Thread[ThreadId(5)]
foo2:0 Thread[ThreadId(7)]
foo2:1 Thread[ThreadId(8)]
foo1:1 Thread[ThreadId(6)]
foo2:2 Thread[ThreadId(9)]
foo1:2 Thread[ThreadId(10)]
foo1:3 Thread[ThreadId(12)]
foo2:3 Thread[ThreadId(11)]
foo2:4 Thread[ThreadId(13)]
foo1:4 Thread[ThreadId(14)]
foo2:5 Thread[ThreadId(16)]
foo1:5 Thread[ThreadId(15)]
foo2:6 Thread[ThreadId(2)]
foo1:6 Thread[ThreadId(4)]
foo2:7 Thread[ThreadId(17)]
foo1:7 Thread[ThreadId(3)]
foo1:8 Thread[ThreadId(7)]
foo2:8 Thread[ThreadId(5)]
foo1:9 Thread[ThreadId(8)]
foo2:9 Thread[ThreadId(6)]
```


## メモ

- `Cargo.toml`に`runtime = "0.3.0-alpha.6"`を入れる必要があって`runtime`がないエラーで少しハマった。
- `await`の一行上に`println!`を仕込むと標準出力だけされて、後続の処理はブロックしてくれる


## References
- [ついに正式対応されるRustのawait/asyncを1\.39betaで確認してみた \- Qiita](https://qiita.com/SoraKumo/items/54a11d5a70cd787d496c)
- [Rustのasync/awaitをスムーズに使うためのテクニック \- Qiita](https://qiita.com/qnighy/items/59133e69a0ba0c6a7fef)