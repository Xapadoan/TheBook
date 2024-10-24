
FROM rust:latest

WORKDIR /usr/src/proj

RUN apt update && apt upgrade -y
RUN apt install mingw-w64 -y

RUN wget https://download.docker.com/linux/static/stable/x86_64/docker-19.03.12.tgz
RUN tar -C /usr/local/bin --strip-components=1 -xf docker-19.03.12.tgz docker/docker
RUN cargo install cross
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-gnu

COPY build-client.sh build-client.sh
RUN chmod +x build-client.sh

CMD ["./build-client.sh"]
