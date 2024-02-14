cargo build
cd tools/
cargo run -r --bin tester ../target/debug/ahc030 < in/0000.txt > out/0000.txt
