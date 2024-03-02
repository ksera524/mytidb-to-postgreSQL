# Rustの公式イメージをベースにする
FROM rust:latest as builder

# Diesel CLIをインストール
RUN cargo install diesel_cli --no-default-features --features mysql

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# ソースコードをコンテナにコピー
COPY . .

# Dieselのセットアップとビルド
RUN diesel setup
RUN diesel migration run
RUN cargo build --release

# 実行段階
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/mytidb-to-postgreSQL /usr/local/bin/mytidb-to-postgreSQL

ARG MYSQL_URL
ARG PG_DATABASE_URL
ENV DATABASE_URL=${MYSQL_URL}
ENV PG_DATABASE_URL=${PG_DATABASE_URL}

# コンテナ起動時にアプリケーションを実行
CMD ["mytidb-to-postgreSQL"]
