#!/bin/bash

cargo build --release
docker build -t lunchbot .
docker compose up -d
sleep 5s
curl localhost:4000/health