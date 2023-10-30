# Rust で Web API をつくる

環境はローカルのコンテナ内で完結させる。

## Mac 利用時の注意点

Homebrew で Rust をインストールすると、フォーミュラが更新されない限りインストール時点のバージョンになります。そして Rust は頻繁に更新します。すると rustup と rustc のバージョンが合わない期間が出てきます。この結果、rust-analyzer の挙動がおかしくなることがあります。うちでは proc macros のエラーでコード内の macro が見つからなくなりました。

まとめ、Rust のインストールは公式のやり方が良い。

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

そして、以下を実行すると解消します。

```shell
rustup update
cargo update
```
