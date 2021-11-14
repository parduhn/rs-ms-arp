#! /bin/bash

git add .
git commit -m "Release"
git push

cargo build --release

echo "Move build to server. Server password needed."
rsync -av --progress --exclude='.*' ./target/release/ms_arp hayo@10.1.2.1:/home/hayo/timeover/ms-rust/ms_arp

cargo set-version --bump patch

