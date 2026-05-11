pkgname=mflash-view
pkgver=0.1.3
pkgrel=1
pkgdesc="A lightweight, native egui viewer for .mflash decks"
arch=('x86_64')
license=('Unlicense')
depends=('gcc-libs' 'fontconfig' 'libx11' 'libxcursor' 'libxrandr' 'libxi' 'speech-dispatcher')
makedepends=('cargo' 'cmake' 'clang')
source=()
sha256sums=()

build() {
  # Force cargo to look at the directory where you ran makepkg
  cd "$startdir"
  cargo build --release --locked
}

package() {
  # Install from your local target folder
  install -Dm755 "$startdir/target/release/mflash-view" "$pkgdir/usr/bin/mflash-view"
  install -Dm644 "$startdir/mflash-view.desktop" "$pkgdir/usr/share/applications/mflash-view.desktop"
  install -Dm644 "$startdir/icon.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/mflash-view.svg"
  # Also install the config so the app finds it
  install -Dm644 "$startdir/config.toml" "$pkgdir/etc/mflash-view/config.toml"
}