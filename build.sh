rm -rf bin
mkdir bin
cp data/opening_book.gam bin
GZIP=-9 tar cvzf bin/opening_book.tar.gz bin/opening_book.gam
rm bin/opening_book.gam
cargo build --release
strip target/release/reversi_rs
cp target/release/reversi_rs bin
