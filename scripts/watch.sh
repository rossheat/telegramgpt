#!/bin/bash

pid=$(lsof -ti:9000)

if [ ! -z "$pid" ]; then
    echo "Killing process on port 9000 with PID $pid"
    kill $pid
fi

cargo lambda watch -a 127.0.0.1 -p 9000
