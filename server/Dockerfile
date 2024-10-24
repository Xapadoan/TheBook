FROM rust:bullseye

RUN apt update && apt upgrade -y
RUN apt install cron -y
RUN echo "15 * * * * ./server/target/release/server --run-tournaments" | crontab -

WORKDIR /the_book

COPY ./shared ./shared
COPY ./server ./server

RUN cd ./server && cargo build --release

RUN chmod +x ./server/server-setup.sh

CMD ["./server/server-setup.sh"]
