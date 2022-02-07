#! /bin/bash


service_restart () {
    export SERVICE=$service

    screen -D -RR $SERVICE -X quit || true
    sleep 5
    screen -dmS $SERVICE sh

    screen -S $SERVICE -X stuff "cd /home/hayo/timeover/services
    "
    screen -S $SERVICE -X stuff "echo $pw | sudo -S ./$service
    "
}

pw=xxxx

for filename in *; do 
    if [[ -x "$filename" ]]
    then
        service=${filename}
        service_restart
    fi;
done

