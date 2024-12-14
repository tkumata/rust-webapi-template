# 🦀 Rust で REST API をつくる

本プロジェクトの目標は、Rust で REST API の雛形を作ることです。私の知識の可能な限りクリーンアーキテクチャを意識しています。

## ✅ 実装済み

- [x] CRUD with Axum
- [x] JWT Token Authentication 🔒

## 🔜 実装予定

- [ ] OAuth2

## 🛠️ 開発環境構築

VS Code Dev Container 拡張をインストールし、以下のファイルを作成することで、ローカルマシンに OCI コンテナを立て Rust 開発環境を構築します。

- .devcontainer/devcontainer.json
- .devcontainer/docker-compose.yml
- .devcontainer/Dockerfile

初回は時間がかかりますが二回目以降は Attach するだけなので時間はかかりません。

## 🔨 ビルド

VS Code 編集画面 `fn main()` の直上の `▶ Run` をクリック。もしくは以下のコマンドを実行。

デバッグ用 ⚙️

```shell
cargo build
```

本番用 📦

```shell
cargo build --release
```

## 💻 使い方

### ユーザ作成

```shell
curl --location --request POST 'http://localhost:4000/user' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Your Name",
    "email": "your_email@example.com",
    "password":"password"
}'
```

### ログイン

```shell
curl --location --request POST 'http://localhost:4000/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "your_email@example.com",
    "password":"password"
}'
```

### ユーザ情報取得

```shell
curl --location 'http://localhost:4000/user' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZTJkMTQyZC01OGZlLTRiOWEtYjZkYi0wMTlmODI0YjRkNTkiLCJleHAiOjE3MzIwMjE5NTl9.l27mHiTGb0Ghx0s1vlQuccb99llcdo-MCSuNMSgRPds'
```

### ユーザ情報編集

```shell
curl --location --request PUT 'http://localhost:4000/user' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZTJkMTQyZC01OGZlLTRiOWEtYjZkYi0wMTlmODI0YjRkNTkiLCJleHAiOjE3MzIwMjE5NTl9.l27mHiTGb0Ghx0s1vlQuccb99llcdo-MCSuNMSgRPds'
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Your Name",
    "email": "new_your_email@example.com"
}'
```

## ⚖️ ライセンス

MIT License.