set shell := ["pwsh.exe", "-c"]

work day:
    cargo watch --clear -w src -x "check --bin {{day}}" -s "just test {{day}}" -s "just lint {{day}}"
work-nc day:
    cargo watch --clear -w src -x "check --bin {{day}}" -s "just test-nc {{day}}" -s "just lint {{day}}"
lint day:
    cargo clippy --bin {{day}}
test day:
    cargo nextest run --bin {{day}}
test-nc day:
    cargo nextest run --no-capture --bin {{day}}
test-all:
    cargo nextest run --no-fail-fast --failure-output=never