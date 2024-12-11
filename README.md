# 使い方
1. `content`配下に記事のディレクトリを用意する
1. `content.md` に記事本文を書く
1. `config.yaml` にタイトル、カテゴリなど書く
1. `cargo run {path}`を実行して投稿する。`{path}`には記事のディレクトリを入れる
 

# memo
- rust の実行環境が壊れてたので[これ](https://stackoverflow.com/questions/55514868/how-to-fix-detected-conflict-error-when-installing-rust-on-windows-10)を参考に直した

## 12/08
- chatgpt の写経になるの嫌なので一旦全部消した
- はてなブログのAPI を検証
- Basic 認証でブログエントリの一覧取得できた
    - 自分が書いた下書きのエントリ?
- hatena_id, password, blog_id は env ファイル的なものに記載して読み込みたい
- ブログエントリの page パラメータを与えた場合は付与してほしい
- エントリ投稿するロジック（一覧取得で形式を確認する）
    

