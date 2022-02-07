#! /bin/bash

start_screen () {
        export SCREEN=$screen
        export WORKDIR=$workdir
        screen -S $SCREEN -p 0 -X stuff "^C"
        sleep 5

        screen -D -RR $SCREEN -X quit || true
        sleep 5
        screen -dmS $SCREEN sh
        screen -S $SCREEN -X stuff "cd ./$SCREEN
        "
}

start_sudo_cargo () {
        screen -S $SCREEN -X stuff "echo $pw | sudo -S cargo run 
        "
}

start_node () {
        screen -S $SCREEN -X stuff "npm start 
        "
}

pw=xxxx

cd ../docker
./start_docker.sh

cd ../rust
screen=ms-arp
start_screen
start_sudo_cargo

cd ../node
screen=to-backend
start_screen
start_node

cd ../react
screen=to-frontend
start_screen
start_node
