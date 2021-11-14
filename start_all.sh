#! /bin/bash


service_restart () {
    export SERVICE=$service

    screen -D -RR $SERVICE -X quit || true
    sleep 5
    screen -dmS $SERVICE sh

    screen -S $SERVICE -X stuff "cd /home/hayo/timeover/services
    "
    screen -S $SERVICE -X stuff "./$service
    "
}

echo 'We will start service as sudo!!'
if [[ "$EUID" = 0 ]]; then
    echo "using root!"
else
    echo "please use: sudo su"
fi

for filename in *; do 
    if [[ -x "$filename" ]]
    then
        service=${filename}
        service_restart
    fi;
done

