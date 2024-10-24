#!/bin/sh

./server/target/release/server --reset-shop
cron
./server/target/release/server --start-server