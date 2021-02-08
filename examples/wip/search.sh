
#!/bin/bash

rm -rf results/*.svg
for size in $(seq 100 25 200); do 
for length in $(seq 80 20 80); do
for force in $(seq 0.8 0.2 1.2); do
for i in {0..10}; do
    ./target/release/examples/wip $i $size $force $length
    cp image.svg results/${i}_${size}_${force}_${length}.svg
done
done
done
done