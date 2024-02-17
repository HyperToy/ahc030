#!/bin/bash

cargo build
cd tools/

case_default=1
if [ $# -eq 0 ]; then
    echo "No arguments provided, using default value: $DEFAULT_VALUE"
    case=$case_default
else
    case=$1
fi
((loop_count = $case - 1))
echo $loop_count
for i in `seq -f '%04g' 0 $loop_count`
do
    echo $i
    cargo run -r --bin tester ../target/debug/ahc030 < in/$i.txt > out/$i.txt
done
