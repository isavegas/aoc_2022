set shell := ["sh", "-cu"]
set dotenv-load
set positional-arguments

year := "2022"
name := "aoc_{{year}}"

alias di := download_input
alias gen := generate
alias t := test
alias wt := watch_test
alias wr := watch_run
alias wc := watch_clippy

# -> list
@default: list

# List just targets
@list:
    just --list --unsorted --list-heading "$(printf 'Targets for {{name}}::\n\r')"

@update_core:
    git -C aoc_core pull origin master

generate day:
    #!/usr/bin/env sh
    DAY="$(printf '%02d' {{day}})"
    IMPL_FILE="{{justfile_directory()}}/src/day/day_$DAY.rs"
    TEST_FILE="{{justfile_directory()}}/tests/day_$DAY.rs"
    IMPL_TEMPLATE="$(cat "{{justfile_directory()}}/templates/day.rs")"
    TEST_TEMPLATE="$(cat "{{justfile_directory()}}/templates/test.rs")"
    check_file() {
        if [ -f "$2" ]; then
            echo "$1 for {{year}} day {{day}} exists at $2"
            return 1
        fi
    }
    check_file Implementation "$IMPL_FILE" \
      && echo "${IMPL_TEMPLATE//@DAY@/$DAY}" > "$IMPL_FILE" \
      && echo "Implementation for day {{day}} generated at $IMPL_FILE"
    check_file Tests "$TEST_FILE" \
      && echo "${TEST_TEMPLATE//@DAY@/$DAY}" > "$TEST_FILE" \
      && echo "Tests for day {{day}} generated at $TEST_FILE"

download_input day:
    #!/usr/bin/env sh
    PADDED_DAY="$(printf '%02d' {{day}})"
    OUT_FILE="{{justfile_directory()}}/src/input/day_$PADDED_DAY.txt"
    if [ -f "$OUT_FILE" ]; then
        echo "Input for {{year}} day {{day}} exists at $OUT_FILE"
        exit 1
    fi
    source "{{justfile_directory()}}/.envrc"
    if [ -z "$AOC_SESSION" ]; then
        echo AOC_SESSION not defined
        exit 1
    fi
    curl -sf -H "Cookie: session=$AOC_SESSION" "https://adventofcode.com/{{year}}/day/{{day}}/input" -o "$OUT_FILE"
    if [ -f "$OUT_FILE" ]; then
        echo "Downloaded {{year}} day {{day}} input to $OUT_FILE"
    else
        echo "Unable to download input!"
    fi

# Platform info
@info:
    echo {{name}} :: {{os()}} {{arch()}}

@run day='' part='':
    cargo run -- run {{if day != "" { "-d " + day } else { "" } }} {{if part != "" { "-p " + part } else { "" } }}

# Watch for code changes, running against input
@watch_run day='' part='':
    cargo watch -c -x 'run -- run {{if day != "" { "-d " + day } else { "" } }} {{if part != "" { "-p " + part } else { "" } }}'

# Watch for code changes, running tests for day
@test day part='':
    cargo nextest run -E "binary(day_$(printf '%02d' {{day}})){{ if part != "" {"& test(part" + part + ")" } else { "" } }}"

# Watch for code changes, running tests for day
@watch_test day part='':
    cargo watch -c -x "nextest run -E 'binary(day_$(printf '%02d' {{day}})) {{if part != "" { " & test(part" + part + ")" } else { "" } }}'"

# Watch for code changes, running cargo clippy
@watch_clippy:
    cargo watch -x clippy
