#! /bin/bash

# global vars
pw=xxxx

# fuunction
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

killscreens () {
    screen -ls | grep Detached | cut -d. -f1 | awk '{print $1}' | xargs kill
}

parameter_check (){
        parameter=$(printf '%s\n' "$1" | awk '{ print toupper($0) }')
        if [ -n "$parameter" ]; 
        then 
                echo Parameter: $parameter
        fi

        if [ "$parameter" == "STOP" ]; 
        then
                killscreens
                exit 0
        fi
}


parameter_check "$@"
# main start

cd ../docker
./start_docker.sh

cd ../rust
screen=ms-arp
start_screen
start_sudo_cargo

cd ../rust
screen=ms-iptables
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
