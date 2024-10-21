FROM rust:bullseye

WORKDIR /the_book

COPY ./shared ./shared
COPY ./server ./server

RUN cd ./server && cargo build --release

CMD ["./server/target/release/server", "--start-server"]
