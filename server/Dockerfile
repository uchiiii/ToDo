FROM rust:1.49-buster AS base

# Install
RUN rustup component add rust-src
RUN cargo install cargo-edit
RUN cargo install diesel_cli --no-default-features --features mysql

# Copy local code to the container image.
WORKDIR /app
COPY . .


FROM base AS build
# Install production dependencies and build a release artifact.
RUN cargo build --release

# production
# FROM rust:1.43.1-slim-stretch
# COPY --from=build-stage /app/target/release/api .
# EXPOSE 8088
# CMD ["/usr/local/bin/api"]