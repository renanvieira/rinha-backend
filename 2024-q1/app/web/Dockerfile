FROM rust:1.76-alpine as build

WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev

COPY . .
RUN cargo install --path .

FROM build as app

WORKDIR /usr/app
COPY --from=build /usr/src/app/target/release/web /usr/app/

CMD ["/usr/app/web"]

