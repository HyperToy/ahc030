- rust binary の生成と実行の方法
    - `cargo build` で、以下のような実行ファイルが生成される。
        - target/debug/<project_name>

- 以下のコマンドでテストツールを実行
```
cargo run -r --bin tester ../target/debug/ahc030 < in/0000.txt > out/0000.txt
```

- ジャッジサーバで TLE が解消しない。
    - [rust で atcoder のインタラクティブな問題やるときの注意](https://qiita.com/butzsuppin/items/ab9d86177a1c46b108d5)
    - proconio の input で詰まっていたらしい。

- test スクリプトで 100ケース回す
    - sh でループ