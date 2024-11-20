#!/bin/sh

cd server

# cargo sqlx migrate run
echo "Migration Done";
cargo build --release
echo "Build OK"

./target/release/server --reset-shop
cron
./target/release/server --start-server
