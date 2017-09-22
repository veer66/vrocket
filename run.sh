#!/bin/bash

while :
do
    if [ x"$PID" != "x" ]
    then
        kill $PID
        sleep 3
    fi
    if `cargo build`
    then
        echo "Run new one"
        cargo run &
        PID=$!
        trap "{ kill $PID; exit 0; }" EXIT
        sleep 2
    fi
    echo "Wait ..."
    inotifywait -r --format '%:e %f' ./src
    echo "Reload ..."
done
