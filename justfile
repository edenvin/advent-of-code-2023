#!/usr/bin/env just --justfile

go day part:
  cargo run --release {{day}} {{part}}

test day:
  cargo watch -x 'test -- day_{{day}}'
