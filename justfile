#!/usr/bin/env just --justfile

go day part:
  cargo run --release {{day}} {{part}}

release:
  cargo build --release    

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1
