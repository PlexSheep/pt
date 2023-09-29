#!/bin/bash
set -e
PKGs=(libpt-{core,math,log,net,bintols,ccc,hedu,bin,py} libpt)
for PKG in "${PKGs[@]}"; do
	echo "Package: $PKG"
	cargo publish --registry cscherr -p "$PKG"
	cargo publish -p "$PKG"
done
