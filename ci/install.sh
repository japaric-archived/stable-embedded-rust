set -euxo pipefail

main() {
    rustup target add $TARGET

    # This install a recent arm-none-eabi-gcc
    local url="https://developer.arm.com/-/media/Files/downloads/gnu-rm/7-2017q4/gcc-arm-none-eabi-7-2017-q4-major-linux.tar.bz2?revision=375265d4-e9b5-41c8-bf23-56cbe927e156?product=GNU%20Arm%20Embedded%20Toolchain,64-bit,,Linux,7-2017-q4-major"
    curl -L $url | tar xj
}

main
