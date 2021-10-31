# build project
cargo build --release
for i in $(seq $1)
do
   echo $i
   # execute generator
   ./target/release/rust-orb-test.exe $i
done
