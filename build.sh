#!/bin/bash

cargo build --release
docker build -t lunchbot .