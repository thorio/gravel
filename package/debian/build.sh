#/usr/bin/env bash
set -e

# TODO: add dependency libwayland-client0 when wayland is ready

reporoot=$(git rev-parse --show-toplevel)
pkgdir="$reporoot/package/debian/pkg"

rm -rf "$pkgdir"

bash "$reporoot/package/package.sh" "$reporoot" "$pkgdir"

install -Dm644 "$reporoot/package/debian/control" "$pkgdir/DEBIAN/control"
(cd $pkgdir/.. && dpkg-deb --build --root-owner-group pkg)
