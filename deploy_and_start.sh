#! /bin/bash

git add .
git commit -m "Release"
git push

cargo build --release

rsync -av --progress --exclude='.*' ./target/release/ms_arp hayo@10.1.2.1:/home/hayo/timeover/ms-rust

cargo set-version --bump patch

