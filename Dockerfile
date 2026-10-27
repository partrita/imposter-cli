FROM rust:1-alpine AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM scratch
COPY --from=build /src/target/release/imposter /imposter
ENTRYPOINT ["/imposter"]
