#!/bin/bash
CRATE_NAME="hello_sqlx"
TARGETS=(x86_64-unknown-linux-gnu x86_64-pc-windows-gnu x86_64-apple-darwin)

mkdir -p install

for TARGET in "${TARGETS[@]}"; do
  echo "Building for $TARGET"
  cross build --target $TARGET --release && cp "target/$TARGET/release/$CRATE_NAME" "install/$CRATE_NAME-$TARGET"
done

echo "Builds finished and binaries are installed."