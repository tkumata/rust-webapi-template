# 🚀 Rust で REST API をつくる

本プロジェクトの目標は、Rust で REST API の雛形を作ることです。私の知識の可能な限りクリーンアーキテクチャを意識しています。

## ✅ 実装済み

- [x] CRUD
- [x] JWT Token

## 🔜 実装予定

- [ ] OAuth2

## 🛠️ 開発環境

VS Code Dev Container 拡張をインストールし、以下のファイルを作成することで、ローカルマシンに OCI コンテナを立て Rust 開発環境を構築します。

- .devcontainer/devcontainer.json
- .devcontainer/docker-compose.yml
- .devcontainer/Dockerfile

初回は時間がかかりますが二回目以降は Attach するだけなので時間はかかりません。

## 💻 Usage

### Create user

```shell
curl --location 'http://localhost:4000/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Your Name",
    "email": "yourmail@example.com",
    "password":"password"
}'
```

### Login

```shell
curl --location 'http://localhost:4000/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "yourmail@example.com",
    "password":"password"
}'
```

### Get user

```shell
curl --location 'http://localhost:4000/users/3e2d142d-58fe-4b9a-b6db-019f824b4d59' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzZTJkMTQyZC01OGZlLTRiOWEtYjZkYi0wMTlmODI0YjRkNTkiLCJleHAiOjE3MzIwMjE5NTl9.l27mHiTGb0Ghx0s1vlQuccb99llcdo-MCSuNMSgRPds'
```

## 📜 License

MIT License.