set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

[default]
[private]
default:
    just --list

# Core Pipeline

[arg("release", long, value="--release")]
build target="std" release="":
    @just _run build {{ target }} {{ release }}

check target="std":
    @just _run check {{ target }} ""

[arg("release", long, value="--release")]
test target="std" release="":
    @just _run test {{ target }} {{ release }}

sanity:
    @just tidy
    @just validate
    @just build std
    @just build no-std
    @just test std
    @just test no-std

# Release
release version:
    cargo +stable set-version {{ version }}
    @just sanity
    @echo "Performing dry run..."
    cargo +stable publish --dry-run --allow-dirty
    git commit -am "chore: release {{ version }}"
    git tag -a "v{{ version }}" -m "Release {{ version }}"
    git push origin main --tags
    cargo +stable publish

# Housekeeping
fmt:
    cargo +nightly fmt --all

fmt-check:
    cargo +nightly fmt --all -- --check

clippy:
    cargo +stable clippy --all-targets --all-features --fix --allow-dirty

lint:
    cargo +stable clippy --all-targets --all-features -- -D warnings

validate:
    @just fmt-check
    @just lint

tidy:
    @just fmt
    @just clippy

# Private helpers

[private]
_run command target release="":
    @just _execute-{{ target }} "{{ command }}" {{ release }}

[private]
_execute-std command release="":
    cargo +stable {{ command }} --all-targets {{ release }}

[private]
_execute-no-std command release="":
    {{ if command == "test" { "cargo +stable test --all-targets --no-default-features " + release } else { "cargo +stable " + command + " --no-default-features --target thumbv7m-none-eabi " + release } }}
