# Rust Web API

## 背景

- Web Router の付け替えが楽にできる Web API を作ってみたい。
- 単純に Rust で TCP/IP するプログラムを作ってみたかった。
- Rust の練習したい。
- 静的型付言語であることとメモリセーフな設計だから Rust には興味あった。

## フレームワーク

今回は比較的新しい Axum を使ってみる。

## 将来のアーキテクチャ

```plaintext
+----------+
| Route 53 |
+----------+

+-----+    +-----+    +--------------+
| ALB |<-->| ECS |<-->| Aurora MySQL |
+-----+    +-----+    +--------------+
```

## 学習中のアーキテクチャ

```plaintext
+---------+   +-------------------+
| Mac     |   | Mac               |
| Browser |-->| Rust Web API:3000 |
+---------+   +-------------------+
or
+---------+   +-----------+
| Mac     |   | Mac       |
| Browser |-->| Container |-------------+
+---------+   +-----------+             |
                    | Ubuntu            |
                    | Rust Web API:3000 |
                    +-------------------+

```

## 要求

app 要求

- 1d6 する API
- CPU Load, Storage Info を返す API
- Threading の確認のため任意の時間 sleep する API

環境要求

- .vscode は意図的に git 管理する。
- .devcontainer も開発環境共通化のために git 管理する。

## API 仕様

| Method | URI           | Return | Notes                       |
|--------|---------------|--------|-----------------------------|
| GET    | /dice         | json   | Return 1d6.                 |
| GET    | /sleep/{:int} | json   | Sleep.                      |
| GET    | /metrics      | json   | Get server metrics.         |
| POST   | /             | json   | Recieve data then use data. |

## Goal

- Axum の使い方に慣れること。
- 簡素な Web API ができてること。
- せめてレイヤードアーキテクチャで。

## Non-Goal

- REST API はまだ。
- クリーンアーキテクチャはまだ。

## Rust の開発環境の準備

VSCode Dev Container で環境構築。以下を `.devcontainer/devcontainer.json` で保存しておけば VSCode でコンテナを管理できる。このコンテナにソースコードを Volume mount してコンテナ内で開発・デバッグできる。

```yaml
{
    "name": "Ubuntu",
    "image": "mcr.microsoft.com/devcontainers/base:jammy",
    "features": {
        "ghcr.io/devcontainers/features/rust:1": {
            "version": "latest",
            "profile": "default"
        }
    }
}
```

```plaintext
[Dev Container 構成概略図]

+-------------+                 +----------------------------+
| Local PC    |                 |  Container                 |
| +--------+  |                 |  +--------+ --> FileSystem |
| | VSCode |---- Exported port --->| VSCode | --> Dev tools  |
| +--------+  |                 |  | Server | --> app        |
|             |                 |  +--------+ --> Debug      |
| +--------+  |                 |  +--------+                |
| | Source |<--- Volume mount ---->| Source |                |
| | code   |  |                 |  | code   |                |
| +--------+  |                 |  +--------+                |
+-------------+                 +----------------------------+
```

### 使用するクレート

- axum
- tokio
- hyper
- tower
- sqlx
