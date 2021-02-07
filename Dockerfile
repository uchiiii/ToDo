FROM rust:1.49-buster AS base

# Install
RUN rustup component add rust-src
RUN cargo install cargo-edit
RUN cargo install diesel_cli --no-default-features --features mysql

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .


FROM base AS build
# Install production dependencies and build a release artifact.
RUN cargo build

# Run the web service on container startup.
# CMD ["ToDo"]