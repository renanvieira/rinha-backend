FROM rust:1.76-alpine as build

WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev curl

COPY . .
RUN cargo install --path .


FROM rust:1.76-alpine as debug

WORKDIR /usr/app

RUN apk add --no-cache musl-dev
COPY . .

RUN cargo install --path .
RUN cargo install --locked tokio-console

CMD ["/usr/app/target/release/web_hyper"]


FROM build as app
RUN apk update
RUN apk add curl

WORKDIR /usr/app
COPY --from=build /usr/src/app/target/release/web_hyper /usr/app/

CMD ["/usr/app/web_hyper"]


