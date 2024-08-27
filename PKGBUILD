pkgname="sali-git"
pkgver=v0.1.0
pkgrel=1
pkgdesc="A customizable greetd frontend using gtk4"
arch=(x86_64)
url="https://github.com/WhySoBad/sali"
license=(MIT)
depends=('gtk4' 'gtk4-layer-shell')
makedepends=(cargo-nightly)
source=("git+https://github.com/WhySoBad/sali.git")
md5sums=('SKIP')
options=(!lto) # see https://github.com/briansmith/ring/issues/1444

# install with name sali instead of sali-git
_binary_name="sali"

pkgver() {
    git describe --long --abbrev=7 --tags | sed 's/\([^-]*-g\)/r\1/;s/-/./g'
}

prepare() {
    export RUSTUP_TOOLCHAIN=nightly
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    export RUSTUP_TOOLCHAIN=nightly
    export CARGO_TARGET_DIR=target

    cargo build --frozen --release
}

package() {
    install -Dm0755 -T "target/release/$_binary_name" "$pkgdir/usr/bin/$_binary_name"
}
