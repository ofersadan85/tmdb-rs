[windows]
set shell := ["pwsh", "-Command"]
[unix]
set shell := ["bash", "-c"]

examples:
    cargo run --example basic
    cargo run --example find_by_imdb_id
    cargo run --example append_to_response
