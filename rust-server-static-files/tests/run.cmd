@echo off
cargo build --release
powershell -Command "cp ./target/release/rust-server-auth.exe ./tests/"
start cmd /c "cd tests & rust-server-auth.exe"
pause
cargo test --release -- --show-output