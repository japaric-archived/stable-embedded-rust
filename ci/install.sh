set -euxo pipefail

main() {
    # This fetches latest stable release of Xargo
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/xargo \
                    | cut -d/ -f3 \
                    | grep -E '^v[0.3.0-9.]+$' \
                    | sort --version-sort \
                    | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
            --force \
            --git japaric/xargo \
            --tag $tag \
            --target x86_64-unknown-linux-musl

    rustup component list | grep 'rust-src.*installed' || \
        rustup component add rust-src

    # This install a recent arm-none-eabi-gcc
    local url="https://developer.arm.com/-/media/Files/downloads/gnu-rm/7-2017q4/gcc-arm-none-eabi-7-2017-q4-major-linux.tar.bz2?revision=375265d4-e9b5-41c8-bf23-56cbe927e156?product=GNU%20Arm%20Embedded%20Toolchain,64-bit,,Linux,7-2017-q4-major"
    curl -L $url | tar xj
}

main
