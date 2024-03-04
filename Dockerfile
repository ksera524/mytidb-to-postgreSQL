# Rustの公式イメージをベースにする
FROM rust:latest as builder

# ビルド引数
ARG MYSQL_URL
ARG PG_DATABASE_URL

# Diesel CLIをインストール（MySQLとPostgreSQLのサポートを追加）
RUN cargo install diesel_cli --no-default-features --features "mysql,postgres"

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# ソースコードをコンテナにコピー
COPY . .


# アプリケーションのビルド
RUN cargo build --release

# 実行段階
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/mytidb-to-postgreSQL /usr/local/bin/mytidb-to-postgreSQL

# 実行時の環境変数を設定
ENV MYSQL_DATABASE_URL=${MYSQL_URL}
ENV PG_DATABASE_URL=${PG_DATABASE_URL}

# コンテナ起動時にアプリケーションを実行
CMD ["mytidb-to-postgreSQL"]
