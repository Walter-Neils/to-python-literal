RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size,panic_immediate_abort" \
  --release
upx --best --lzma target/release/to-python-literal
