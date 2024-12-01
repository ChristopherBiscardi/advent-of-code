# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"
lint day:
    cargo clippy -p {{day}}
test day part:
    cargo nextest run -p {{day}} {{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}}-bench {{part}} >> {{day}}.bench.txt
# create the directory for a new day's puzzle and fetch the input
create day:
    cargo generate --path ./daily-template --name {{day}}
    just get-input {{day}}

# You can find SESSION by using Chrome tools:
# 1) Go to https://adventofcode.com/2022/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session. Fill it into your .env file
# 
# example .env:
#
# ```
# SESSION=PASTE_COOKIE_VALUE_HERE
# ```
#
# get the input for a day's puzzle
get-input day:
    ./scripts/get-aoc-input.rs --day {{day}} --current-working-directory {{justfile_directory()}}
