#!/bin/sh


build_client() {
    TARGET=$1
    FILENAME=$2
    echo "Start building client for $TARGET"

    cd client
    cargo build --target $TARGET --release
    echo "Client build done"

    cd ..
    mkdir $TARGET
    cp target/$TARGET/release/$FILENAME $TARGET/
    cp client/.env.client.example $TARGET/.env.client

    tar -cf $TARGET.tar $TARGET/$FILENAME $TARGET/.env.client
    rm -rf $TARGET
}

build_client x86_64-pc-windows-gnu client.exe
build_client x86_64-unknown-linux-gnu client
