FROM rust:bullseye

RUN apt update && apt upgrade -y
RUN apt install cron -y
RUN apt install libssl-dev -y
RUN echo "0,15,30,45 * * * * cd /the_book && /the_book/server/target/release/server --run-tournaments" | crontab -

WORKDIR /the_book

RUN cargo install sqlx-cli

COPY ./shared ./shared
COPY ./server ./server

RUN chmod +x ./server/server-setup.sh

CMD ["./server/server-setup.sh"]
