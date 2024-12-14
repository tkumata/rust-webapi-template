# DB migration

DB migration について。

## 💻 Deisel の使い方

今後 DB に手を入れる場合の操作方法について。初期構築は後述。

### ✨ migration ファイルの作成

以下を実行して migration ファイルの雛形を作成。

```shell
diesel migration generate sql_exec_name
```

以下の 2 つのファイルが `migrations/<timestamp>_<sql_exec_name>` ディレクトリ内に生成される。

- up.sql (migration 適用に関する SQL を記述)
- down.sql (migration 取り消しに関する SQL を記述)

### 📝 migration ファイルの編集

上記ファイルを編集する。

### ⚙️ migration 実行

上記 migration ファイルを DB に適用する。

```shell
diesel migration run
```

### 👀 migration 確認

```shell
diesel migration list
```

以下のようになる。

```shell
Migrations:
  [X] 00000000000000_diesel_initial_setup
  [X] 2024-11-23-043245_create_users
```

## 🛠 Diesel 初期構築

コンテナを作り直したときなど以下を実行。

```shell
cargo install diesel_cli --no-default-features --features "postgres"
```

本当に初めてなら以下も実行。disel.toml と migrations ディレクトリが存在すれば実行しなくて良い。

```shell
diesel setup
```
