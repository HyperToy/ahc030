#!/bin/bash

cargo build -r
cd tools/
mkdir -p vis
cargo build -r

case_default=50
if [ $# -eq 0 ]; then
    # echo "No arguments provided, using default value: $case_default"
    case=$case_default
else
    case=$1
fi
((loop_stop = $case - 1))

echo $case > score.txt

for i in `seq -f '%04g' 0 $loop_stop`
do
    ./target/release/tester ../target/release/ahc030 < in/$i.txt > out/$i.txt 2>> score.txt
    ./target/release/vis  in/$i.txt out/$i.txt > /dev/null && mv vis.html vis/$i.html
done

cd ..
scores=scores.txt
date "+%Y-%m-%d %H:%M:%S" >> $scores
python ./calc.py < tools/score.txt >> $scores
echo $null >> $scores

rm tools/score.txt

afplay /System/Library/Sounds/Hero.aiff
