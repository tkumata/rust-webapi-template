# Rust で Web API をつくる

本プロジェクトの目標は、Rust で REST API の雛形を作ることです。私の知識の可能な限りクリーンアーキテクチャを意識しています。

- [x] CRUD できる。
- [x] JWT token を用いて認証済みのみアクセスさせる。

## 開発環境

VS Code Dev Container でローカルマシン内に OCI コンテナを立て Rust 開発ツールを導入します。

.devcontainer ディレクトリ内に以下のファイルを作成します。

- devcontainer.json
- docker-compose.yml
- Dockerfile

初回は時間がかかりますが二回目以降は Attach するだけなので時間はかかりません。

## Usage

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
