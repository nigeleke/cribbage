FROM rust:1 AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH" SQLX_OFFLINE=true
RUN \
    dx bundle --package=web --web --release &\
    dx bundle --package=web --server --release

FROM alpine:latest AS runtime
WORKDIR /app
COPY --from=builder --chown=1000:1000 /app/target/dx/web/release/web/ .
USER 1000:1000
ENV IP=0.0.0.0 PORT=8080 RUST_LOG=info
EXPOSE 8080
CMD ["./web"]
