#!/bin/bash
set -e
target=aarch64-horizon-elf
type=nro

CARGO_INCREMENTAL=0 RUST_TARGET_PATH=$PWD RUST_BACKTRACE=1 xargo build --release --target $target

for elf in $PWD/target/$target/release/*.elf; do
    outelf=$elf
    break 1
done

if [ "$type" == "elf" ]; then
    elfname=$(basename -- "$PWD/$outelf")
    newelf=$PWD/$elfname
    cp -r $elf $newelf
elif [ "$type" == "nro" ]; then
    elfname=$(basename -- "$PWD/$outelf")
    outnro="$PWD/${elfname%.*}".nro
    /opt/devkitpro/tools/bin/elf2nro $elf $outnro
    echo "Generated NRO: $outnro"
elif [ "$type" == "nso" ]; then
    elfname=$(basename -- "$PWD/$outelf")
    outnso="$PWD/${elfname%.*}".nso
    /opt/devkitpro/tools/bin/elf2nso $elf $outnso
    echo "Generated NSO: $outnso"
fi