#! /bin/bash


service_restart () {
    export SERVICE=$service

    screen -D -RR $SERVICE -X quit || true
    sleep 5
    screen -dmS $SERVICE sh

    screen -S $SERVICE -X stuff "cd /home/hayo/timeover/services
    "
    screen -S $SERVICE -X stuff "$service
    "
}

cd /home/hayo/timeover/services
for filename in *; do 
    service=${filename}
    service_restart; 
done

