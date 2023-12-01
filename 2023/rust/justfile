work day part:
    cargo watch -x "check -p {{day}}" -s "just test {{part}} -p {{day}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}" -s "just flamegraph {{day}} {{part}}"
www-watch:
   RUST_LOG=info cargo +nightly leptos watch --project www
www-build:
   cargo +nightly leptos build --project www --release
lint day:
    cargo clippy -p {{day}}
test part +FLAGS='-p day-01':
    cargo nextest run {{FLAGS}} {{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}} {{part}} >> {{day}}.bench.txt
flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package {{day}} --bin {{part}} -o flamegraphs/{{day}}--{{part}}.svg
dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin {{part}}
create day:
    cargo generate --path ./daily-template --name {{day}}
