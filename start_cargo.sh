#! /bin/bash

start_screen () {
        export SCREEN=$screen
        export WORKDIR=$workdir
        screen -S $SCREEN -p 0 -X stuff "^C"
        sleep 20

        screen -D -RR $SCREEN -X quit || true
        sleep 10
        screen -dmS $SCREEN sh
        screen -S $SCREEN -X stuff "cd ./$SCREEN
        "
        screen -S $SCREEN -X stuff "echo $pw | sudo -S cargo run 
        "
}
pw=xxxx

cd ../docker
./start_docker.sh

cd ../rust
screen=ms-arp
start_screen

