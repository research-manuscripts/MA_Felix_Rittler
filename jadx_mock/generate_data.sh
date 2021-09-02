cargo build --release
for i in $(seq $1)
do
   ./target/release/rust-orb-test.exe $i
done
