cargo build --release
for i in {1..10}
do
   ./target/release/rust-orb-test.exe $i
done
