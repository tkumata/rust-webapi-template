# Deisel の使い方

今後 DB に手を入れる場合の操作方法について。初期構築の手順は省略。

## migration ファイルの作成

以下を実行して migration ファイルの雛形を作成。

```shell
diesel migration generate create_users
```

以下の 2 つのファイルが `migrations/<timestamp>_create_users` ディレクトリ内に生成される。

- up.sql (migration 適用に関する SQL を記述)
- down.sql (migration 取り消しに関する SQL を記述)

## migration ファイルの編集

上記ファイルを編集する。

## migration 実行

上記 migration ファイルを DB に適用する。

```shell
diesel migration run
```