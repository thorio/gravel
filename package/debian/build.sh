#/usr/bin/env bash
set -e

reporoot=$(git rev-parse --show-toplevel)
pkgdir="$reporoot/package/debian/pkg"

rm -rf "$pkgdir"

bash "$reporoot/package/package.sh" "$reporoot" "$pkgdir"

install -Dm644 "$reporoot/package/debian/control" "$pkgdir/DEBIAN/control"
(cd $pkgdir/.. && dpkg-deb --build --root-owner-group pkg)
