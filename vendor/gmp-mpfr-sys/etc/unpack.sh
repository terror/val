#!/bin/bash

# Copyright © 2017–2026 Trevor Spiteri

# Copying and distribution of this file, with or without modification,
# are permitted in any medium without royalty provided the copyright
# notice and this notice are preserved. This file is offered as-is,
# without any warranty.

# This script untars gmp, mpfr, mpc and tweaks them a little.

set -e

# Change the variables below before running

# library versions and tar locations
TARDIR="$HOME/Downloads"

GMPVER=6.3.0
GMPVERP="$GMPVER"
GMPTAR="$TARDIR/gmp-$GMPVER.tar.xz"

MPFRVER=4.2.2
MPFRVERP="$MPFRVER"
MPFRTAR="$TARDIR/mpfr-$MPFRVER.tar.xz"

MPCVER=1.4.1
MPCVERP="$MPCVER"
MPCTAR="$TARDIR/mpc-$MPCVER.tar.xz"

CHANGELOG_CHARS=100000

# if in etc directory, change to upper directory
if [ -e unpack.sh ]; then
	cd ..
fi

function truncate {
	mv "$1" "$1.rm~"
	(
		if (($2 > 0)); then
			head -c $(($2 - 1))
			head -n 1
		fi
		if [ $(head -c 1 | wc -c) == 1 ]; then
			echo "... (truncated)"
		fi
	) < "$1.rm~" > "$1"
}

## GMP
if [ -e gmp-*-c ]; then
	rm -r gmp-*-c
fi
tar xf "$GMPTAR"
mv gmp-$GMPVER gmp-$GMPVERP-c
cd gmp-$GMPVERP-c
for p in "../etc/gmp-$GMPVER-p"*; do
    if [ -f "$p" ]; then
	patch -N -Z -p1 --no-backup-if-mismatch < "$p" > /dev/null
    fi
done
# Truncate ChangeLog
truncate ChangeLog $CHANGELOG_CHARS
# Remove doc/*.info*, doc/*.tex
rm doc/*.info* doc/*.tex
# a. Remove demos section in configure
# b. Remove doc/Makefile, demos/{,*/}Makefile from ac_config_files in configure
sed -i.rm~ -e '
    /Configs for demos/,/Create config.m4/{
        /Create config.m4/!s/^/#gmp-mpfr-sys /
        s/\(#gmp-mpfr-sys\) $/\1/
    }
    /^ac_config_files=/{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(\(doc\|demos[/a-z]*\)/Makefile\)\([^#]*\)\($\| #\),\1\4 #gmp-mpfr-sys \2\5,
        t repeat
    }
' configure
# Remove doc and demos from SUBDIRS in Makefile.in
sed -i.rm~ -e '
    /^SUBDIRS = /{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(doc\|demos\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
        t repeat
    }
' Makefile.in
# In tests/misc/t-locale.c, add " && ! defined __ANDROID__" to "#if HAVE_NL_LANGINFO".
sed -i.rm~ -e 's/#if HAVE_NL_LANGINFO/& \&\& ! defined __ANDROID__/' tests/misc/t-locale.c
# Include a Rust entry in the Language Bindings documentation.
sed -i.rm~ -e '/@item Scheme/ {
    i\
@item Rust
    i\
@itemize @bullet
    i\
@item
    i\
Rug @spaceuref{https://crates.io/crates/rug}
    i\
@end itemize
    i\

}' doc/gmp.texi
# Patch longlong.h to not use mulx, as it is not supported by all x86 architectures.
sed -i.rm~ 's/#if X86_ASM_MULX /&\&\& 0 /' longlong.h
cd ..

## MPFR
if [ -e mpfr-*-c ]; then
    rm -r mpfr-*-c
fi
tar xf "$MPFRTAR"
mv mpfr-$MPFRVER mpfr-$MPFRVERP-c
cd mpfr-$MPFRVERP-c
for p in "../etc/mpfr-$MPFRVER-p"*; do
    if [ -f "$p" ]; then
	patch -N -Z -p1 --no-backup-if-mismatch < "$p" > /dev/null
    fi
done
# Truncate ChangeLog
truncate ChangeLog $CHANGELOG_CHARS
# Remove doc/*.info*, doc/*.tex
rm doc/*.info* doc/*.tex
# Remove doc/Makefile, mpfr.pc from ac_config_files in configure
sed -i.rm~ -e '
    /^ac_config_files=/{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(doc/Makefile\|mpfr.pc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
        t repeat
    }
' configure
# a. Remove doc from SUBDIRS in Makefile.in
# b. Remove $(pkgconfig_DATA) from DATA in Makefile.in
sed -i.rm~ -e '
    /^SUBDIRS = /s,^\([^#]*\) \(doc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
    /^DATA = /s,^\([^#]*\) \(\$(pkgconfig_DATA)\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' Makefile.in
# Remove get_patches.c rule in src/Makefile.in
sed -i.rm~ '/get_patches.c:/,/^$/s/^\(.\)/#gmp-mpfr-sys \1/' src/Makefile.in
# Generate src/get_patches.c
tools/get_patches.sh PATCHES > src/get_patches.c
cd ..

## MPC
if [ -e mpc-*-c ]; then
	rm -r mpc-*-c
fi
tar xf "$MPCTAR"
mv mpc-$MPCVER mpc-$MPCVERP-c
cd mpc-$MPCVERP-c
for p in "../etc/mpc-$MPCVER-p"*; do
    if [ -f "$p" ]; then
	patch -N -Z -p1 --no-backup-if-mismatch < "$p" > /dev/null
    fi
done
# Make sure all files are user writeable
chmod -R u+w *
# Truncate ChangeLog
truncate ChangeLog $CHANGELOG_CHARS
# Remove doc/*.info*, build-aux/*.tex
rm doc/*.info* build-aux/*.tex
# Remove doc/Makefile from ac_config_files in configure
sed -i.rm~ '
    /^ac_config_files=/s,^\([^#]*\) \(doc/Makefile\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' configure
# Remove doc from SUBDIRS in Makefile.in
sed -i.rm~ '
    /^SUBDIRS = /s,^\([^#]*\) \(doc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' Makefile.in
cd ..

## Comment Makefile:...esac sections from all Makefile.in
for m in $(find *-c -name Makefile.in); do
    sed -i.rm~ '/Makefile:/,/esac/s/^/#gmp-mpfr-sys /' $m
done

## Documentation
if [ -e doc-c ]; then
    rm -r doc-c
fi
# Build html documentation
mkdir doc-c{,/GMP,/MPFR,/MPC}
makeinfo gmp*/doc/gmp.texi --html --split=chapter --output=doc-c/GMP
makeinfo mpfr*/doc/mpfr.texi --html --split=chapter --output=doc-c/MPFR
makeinfo mpc*/doc/mpc.texi --html --split=chapter --output=doc-c/MPC
for f in doc-c/*/*.html; do
    # Remove unnecessary node redirects
    if grep -q 'The node you are looking for is' "$f"; then
        rm "$f"
        continue
    fi
    # a. Remove anything outside <body>, including the <body> and </body> tags themselves
    # b. Remove blank lines (so that rustdoc's markdown interpreter sees as html)
    sed -i.rm~ -e '
        0,/<body/{
            /<!-- This manual/,/-->/!d
        }
    	/<\/body>/,$d
        /^$/d
    ' "$f"
    # Remove stray backslashes
    sed -i.rm~ -e '
        s/mp\\_bits\\_per\\_limb/mp_bits_per_limb/g
        s/GMP\\_NUMB\\_BITS/GMP_NUMB_BITS/g
        s/\\log/log/g
        s/\\exp/exp/g
        s/\\pi/Pi/g
        s/\\infty/Inf/g
    ' "$f"
    # Clear margins and padding for tables with class="menu", "index-cp", "index-fn"
    sed -i.rm~ -e '
        /<table class="menu"/,/<\/table>/s/<td\|<th/& style="padding: 0; border: 0;" /g
	/<table class="index-/,/<\/table>/s/<td\|<th/& style="padding: 1px; border: 0;" /g
	s/<table class="\(menu\|index-[cpfn]*\)"/& style="margin: 0; width: auto; padding: 0; border: 0;"/
    ' "$f"
    # Redirect links by
    # a. replacing directory links by "../../index.html"
    # b. appending "#start" to links having no slashes and ending in ".html"
    # c. prepending "constant."
    # d. replacing "-" and "_002d" by "_"
    # e. replacing "_002b" by "P"
    sed -i.rm~ -e '
        s/..\/dir\/index.html\|dir.html#Top/..\/..\/index.html/g
        s/\(href="[^/"]*\.html\)"/\1#start"/g
        :repeat
        s/\(href="\)\([A-Z][A-Za-z0-9_-]*\.html\)/\1constant.\2/
        t repeat
        s/\("constant\.[A-Za-z0-9_]*\)-\([A-Za-z0-9_-]*\.html\)/\1_\2/
        t repeat
        s/\("constant\.[A-Za-z0-9_]*\)_002b\([A-Za-z0-9_]*\.html\)/\1P\2/
        t repeat
        s/\("constant\.[A-Za-z0-9_]*\)_002d\([A-Za-z0-9_]*\.html\)/\1_\2/
        t repeat
    ' "$f"
done

## Remove all .rm~ files left over by sed
find *-c -name \*.rm~ -delete
