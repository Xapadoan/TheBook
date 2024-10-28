#!/bin/sh

cd server

cargo sqlx migrate run
cargo build --release

./target/release/server --reset-shop
cron
./target/release/server --start-server
