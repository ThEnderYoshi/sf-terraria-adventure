:: Performs a pack_diagnostic scan.
:: Run this from the repo's root dir.

CD Tools\pack_diagnostic
cargo run -- scan -i ../../Content -o ../generated_refs
