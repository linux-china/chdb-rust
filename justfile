### build
build:
  cargo build

### Run basic example
run-basic:
  cargo run --example basic

### Use clickhouse local to run query
local-query:
  clickhouse local --query "SELECT leftPad('abc', 7, ' ')"

### setup: download chdb library
setup:
  rm -rf chdb.h
  rm -rf libchdb.so
  ./update_libchdb.sh