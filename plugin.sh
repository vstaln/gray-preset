#!/bin/sh
here="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
sys="$(uname -s)"
case "$sys" in
MINGW* | MSYS* | CYGWIN*) rel="x86_64-pc-windows-msvc/gray-preset.exe" ;;
Darwin*)
    case "$(uname -m)" in
    arm64) rel="aarch64-apple-darwin/gray-preset" ;;
    *) rel="x86_64-apple-darwin/gray-preset" ;;
    esac ;;
*)
    case "$(uname -m)" in
    aarch64 | arm64) rel="aarch64-unknown-linux-musl/gray-preset" ;;
    *) rel="x86_64-unknown-linux-musl/gray-preset" ;;
    esac ;;
esac
if [ -x "$here/$rel" ]; then exec "$here/$rel" "$@"; fi
for cand in "$here/target/release/gray-preset.exe" "$here/target/release/gray-preset" "$here/target/debug/gray-preset.exe" "$here/target/debug/gray-preset"; do
    if [ -x "$cand" ]; then exec "$cand" "$@"; fi
done
echo "gray-preset binary not found in $here" >&2
exit 1
