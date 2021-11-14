#! /bin/bash

git add .
git commit -m "Release"
git push

cargo build --release

echo "Move build to server. Server password needed."
cd ./target/release
rsync -avR  --progress --exclude='.*' ms_arp hayo@10.1.2.1:/home/hayo/timeover/services/
cd ../..
rsync -av  --progress --exclude='.*' start_ms hayo@10.1.2.1:/home/hayo/timeover/services/

echo "Start service with /timeover/services/start_all.sh"

cargo set-version --bump patch

