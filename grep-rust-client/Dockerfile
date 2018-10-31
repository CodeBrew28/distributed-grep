FROM rust:1.24.0

WORKDIR /usr/src/app
COPY . .

RUN cargo install

CMD ["grep-server"]