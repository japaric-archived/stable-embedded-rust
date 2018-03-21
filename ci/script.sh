set -euxo pipefail

# check that everything *links* with different optimization profiles
main() {
    local examples=(
        asm
        exception
        intrinsics
        mem
        minimal
    )

    for ex in "${examples[@]}"; do
        xargo build --target $TARGET --example $ex

        xargo build --target $TARGET --example $ex --release

        xargo rustc --target $TARGET --example $ex --release -- -C lto
    done

    set +x
    IFS=,;eval arm-none-eabi-size target/$TARGET/debug/examples/"{${examples[*]}}"

    echo

    IFS=,;eval arm-none-eabi-size target/$TARGET/release/examples/"{${examples[*]}}"
}

main
