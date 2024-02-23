FROM rust:1.76-alpine as build

WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev

COPY . .
RUN cargo install --path .


FROM rust:1.76-alpine as debug

WORKDIR /usr/app

RUN apk add --no-cache musl-dev
COPY . .

RUN RUSTFLAGS="--cfg tokio_unstable" cargo install --path .
RUN RUSTFLAGS="--cfg tokio_unstable" cargo install --locked tokio-console

CMD ["/usr/app/target/release/web"]


FROM build as app

WORKDIR /usr/app
COPY --from=build /usr/src/app/target/release/web /usr/app/

CMD ["/usr/app/web"]


