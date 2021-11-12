FROM rust:1.55.0

WORKDIR /actix-app

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y ca-certificates && update-ca-certificates
RUN apt-get install -y default-mysql-client

RUN cargo install diesel_cli --no-default-features --features mysql