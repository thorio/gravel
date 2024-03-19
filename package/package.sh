#/usr/bin/env bash
set -e

reporoot=$1
pkgdir=$2

for file in $reporoot/package/icons/x*.png; do
	size=$(echo $file | grep -Po '\d+')
	path="$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps"

	install -Dm644 "$file" "$path/gravel.png"
done

install -Dm755 "$reporoot/target/release/gravel" "$pkgdir/usr/bin/gravel"
install -Dm644 "$reporoot/package/gravel.desktop" "$pkgdir/usr/share/applications/gravel.desktop"
install -Dm644 "$reporoot/config.yml" "$pkgdir/usr/share/doc/gravel/default-config.yml"
