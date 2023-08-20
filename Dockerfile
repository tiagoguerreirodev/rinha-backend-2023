FROM rust:1.71.1-slim-bullseye
LABEL authors="Tiago Guerreiro"

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["rinha-backend-2023"]