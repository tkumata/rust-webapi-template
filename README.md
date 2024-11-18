# Rust で Web API をつくる

本プロジェクトの目標は、Rust で REST API の雛形を作ることです。私の知識の可能な限りクリーンアーキテクチャを意識しています。

## 開発環境

VSCode Dev Container でローカルマシン内に OCI コンテナを立て Rust 開発ツールを導入します。

.devcontainer ディレクトリ内に以下のファイルを作成します。

- devcontainer.json
- docker-compose.yml
- Dockerfile

初回は時間がかかりますが二回目以降は Attach するだけなので時間はかかりません。

devcontainer.json

```json
{
	"name": "Rust and PostgreSQL",
	"dockerComposeFile": "docker-compose.yml",
	"service": "app",
	"workspaceFolder": "/workspaces/${localWorkspaceFolderBasename}",
	"forwardPorts": [5432]
}
```

docker-compose.yml

```yaml
version: '3.8'

volumes:
  postgres-data:

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
    env_file:
        - .env
    volumes:
      - ../..:/workspaces:cached
    command: sleep infinity
    network_mode: service:db

  db:
    image: postgres:14.1
    restart: unless-stopped
    volumes:
      - postgres-data:/var/lib/postgresql/data
    env_file:
      - .env
```

Dockerfile

```Dockerfile
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y
```

## Mac 利用時の注意点

Homebrew で Rust をインストールすると、フォーミュラが更新されない限りインストール時点のバージョンになります。そして Rust は頻繁に更新します。すると rustup と rustc のバージョンが合わない期間が出てきます。この結果、rust-analyzer の挙動がおかしくなることがあります。うちでは proc macros のエラーでコード内の macro が見つからなくなりました。

もしコンテナを使わずに Mac に Rust のインストールするなら、[公式](https://www.rust-lang.org/tools/install)のやり方が良いとです。

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

そして、Rust のバージョンアップは以下でできます。

```shell
rustup update
```

プロジェクト内のクレートなどのバージョンアップは以下でできます。

```shell
cd "your project"
cargo update
```

Homebrew で Rust をインストールすると依存関係でその他のソフトウェアもインストールされます。うちでは llvm と zx が入り、これらが他のソフトウェアの依存先にもなってしまい綺麗にするのに手間がかかりました。そういった意味でも初手公式が望ましいです。
