#! /bin/bash
service=$(cat Cargo.toml | grep name -m 1 | awk -F' = ' '{ print $2 }' | sed 's/.\(.*\)/\1/' | sed 's/\(.*\)./\1/')

git add .
git commit -m "Release"
git push

cargo build --release

echo "Move build to server. Server password needed."
cd ./target/release
rsync -avR  --progress --exclude='.*' $service hayo@10.1.2.1:/home/hayo/timeover/services/
cd ../..

#rsync -av  --progress --exclude='.*' start_all.sh hayo@10.1.2.1:/home/hayo/timeover/services/

echo "Start service with /timeover/services/start_all.sh"

cargo set-version --bump patch

#echo "Service start cargo"
#cp ./start_all_dev.sh ../../timeover/start_all_dev.sh
#chmod a+a ../../timeover/start_all_dev.sh
