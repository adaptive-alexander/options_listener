# stage 1 - cache skeleton
FROM rust AS chef
WORKDIR /app
RUN apt update && apt install -yq cmake git && \
    cargo install cargo-chef

# stage 2 - use cached deps
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 3 - build
FROM chef AS builder
COPY --from=planner app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# copy app into the docker
COPY . .
# build app
RUN cargo install --path .

# use google distroless as runtime image
FROM gcr.io/distroless/cc-debian11
# copy app from builder
COPY --from=builder /usr/local/cargo/bin/options_listener /app/options_listener
# set work dir in second image
WORKDIR /app
# start app
ENTRYPOINT ["/app/options_listener"]
