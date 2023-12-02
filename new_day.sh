#!/usr/bin/env bash

# call with ./new_day.sh 02

cp -R day-00 day-$1
sed -i "s/day-00/day-$1/" day-$1/Cargo.toml
