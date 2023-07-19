#!/bin/bash

cargo build --release
docker build -t lunchbot .
docker compose up -d
curl localhost:4000/health