<!-- Copyright © 2017–2026 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

Version 1.7.1 (2026-04-25)
==========================

  * [MPC] was updated from version 1.4.0 to 1.4.1.

Version 1.7.0 (2026-03-19)
==========================

  * The crate now requires rustc version 1.71.0 or later.
  * [MPC] was updated from version 1.3.1 to 1.4.0.

Version 1.6.8 (2025-09-03)
==========================

  * Bug fix: build was failing for some gcc versions ([issue 43], [merge request
    6]).

[issue 43]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/43
[merge request 6]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/merge_requests/6

Version 1.6.7 (2025-08-21)
==========================

  * [GMP] was failing to compile on GCC 15 ([issue 39]).

[issue 39]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/39

Version 1.6.6 (2025-03-21)
==========================

  * Do not use the `mulx` instruction in [GMP] as it does not support all
    architectures ([Rug issue 79]).

[Rug issue 79]: https://gitlab.com/tspiteri/rug/-/issues/79

Version 1.6.5 (2025-03-20)
==========================

  * [MPFR] was updated from version 4.2.1 to 4.2.2.
  * 32-bit MSYS2 MinGW is no longer tested and mentioned in the documentation.

Version 1.6.4 (2024-06-05)
==========================

  * The license headers in the documentation for the GNU libraries were being
    inadvertently removed from the html files ([issue 34]).

[issue 34]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/34

Version 1.6.3 (2024-05-30)
==========================

  * Make cross-compiling to MinGW use a host understood by [GMP] ([merge request
    5]).

[merge request 5]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/merge_requests/5

Version 1.6.2 (2024-01-20)
==========================

  * Bug fix: when using system libs, the build would fail for some cases ([issue
    32]).

[issue 32]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/32

Version 1.6.1 (2023-08-23)
==========================

  * [MPFR] was updated from version 4.2.0-p12 to 4.2.1.

Version 1.6.0 (2023-07-30)
==========================

  * [GMP] was updated from version 6.2.1 to 6.3.0.
  * [MPFR] was updated from version 4.2.0-p9 to 4.2.0-p12.

Version 1.5.3 (2023-05-22)
==========================

  * [MPFR] was updated from version 4.2.0 to 4.2.0-p9.

Version 1.5.2 (2023-03-26)
==========================

  * Bug fix: <code>[mpfr][mpfr-1-5]::[pow\_sj][mpfr-ps-1-5]</code> and
    <code>[mpfr][mpfr-1-5]::[pow\_uj][mpfr-pu-1-5]</code> were linking
    to the wrong MPFR function name ([issue 30]).
  * The commits [`0216f40ed..a041c7cbb`] from the [4.2 branch][mpfr 4.2 branch]
    of [MPFR] were merged.

[`0216f40ed..a041c7cbb`]: https://gitlab.inria.fr/mpfr/mpfr/-/compare/0216f40ed...a041c7cbb
[issue 30]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/30
[mpfr-ps-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.pow_sj.html
[mpfr-pu-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.pow_uj.html

Version 1.5.1 (2023-02-28)
==========================

  * The commits [`01a3ed526..78ff7526d`] and [`80ea348b3..0216f40ed`] from the [4.2
    branch][mpfr 4.2 branch] of [MPFR] were merged.

[`01a3ed526..78ff7526d`]: https://gitlab.inria.fr/mpfr/mpfr/-/compare/01a3ed526...78ff7526d
[`80ea348b3..0216f40ed`]: https://gitlab.inria.fr/mpfr/mpfr/-/compare/80ea348b3...0216f40ed
[mpfr 4.2 branch]: https://gitlab.inria.fr/mpfr/mpfr/-/tree/4.2

Version 1.5.0 (2023-01-06)
==========================

  * The crate now requires rustc version 1.65.0 or later.
  * [MPFR] was updated from version 4.1.1-p1 to 4.2.0.
  * [MPC] was updated from version 1.2.1 to 1.3.1.
  * The following functions are now `const` functions:
      * <code>[gmp][gmp-1-5]::[mpz\_sgn][gmp-zs-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpq\_sgn][gmp-qs-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpf\_sgn][gmp-fs-1-5]</code>
      * <code>[gmp][gmp-1-5]::[mpz\_odd\_p][gmp-zop-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpz\_even\_p][gmp-zep-1-5]</code>
      * <code>[gmp][gmp-1-5]::[mpz\_fits\_ulong\_p][gmp-zfulp-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpz\_fits\_uint\_p][gmp-zfuip-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpz\_fits\_ushort\_p][gmp-zfusp-1-5]</code>
      * <code>[gmp][gmp-1-5]::[mpz\_getlimbn][gmp-zg-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpz\_size][gmp-zsi-1-5]</code>,
      * <code>[gmp][gmp-1-5]::[mpq\_numref][gmp-qn-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpq\_numref\_const][gmp-qnc-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpq\_denref][gmp-qd-1-5]</code>,
        <code>[gmp][gmp-1-5]::[mpq\_denref\_const][gmp-qdc-1-5]</code>
      * <code>[mpfr][mpfr-1-5]::[get\_prec][mpfr-gp-1-5]</code>,
      * <code>[mpfr][mpfr-1-5]::[nan\_p][mpfr-nap-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[inf\_p][mpfr-ip-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[number\_p][mpfr-nup-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[zero\_p][mpfr-zp-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[regular\_p][mpfr-rp-1-5]</code>
      * <code>[mpfr][mpfr-1-5]::[get\_exp][mpfr-ge-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[signbit][mpfr-s-1-5]</code>
      * <code>[mpfr][mpfr-1-5]::[VERSION\_NUM][mpfr-vn-1-5]</code>
      * <code>[mpfr][mpfr-1-5]::[custom\_get\_size][mpfr-cgs-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[custom\_init][mpfr-ci-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[custom\_get\_kind][mpfr-cgk-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[custom\_get\_significand][mpfr-cgsi-1-5]</code>,
        <code>[mpfr][mpfr-1-5]::[custom\_get\_exp][mpfr-cge-1-5]</code>
      * <code>[mpc][mpc-1-5]::[INEX\_RE][mpc-ir-1-5]</code>,
        <code>[mpc][mpc-1-5]::[INEX\_IM][mpc-ii-1-5]</code>,
        <code>[mpc][mpc-1-5]::[INEX1][mpc-i1-1-5]</code>,
        <code>[mpc][mpc-1-5]::[INEX2][mpc-i2-1-5]</code>
      * <code>[mpc][mpc-1-5]::[realref][mpc-r-1-5]</code>,
        <code>[mpc][mpc-1-5]::[realref\_const][mpc-rc-1-5]</code>,
        <code>[mpc][mpc-1-5]::[imagref][mpc-i-1-5]</code>,
        <code>[mpc][mpc-1-5]::[imagref\_const][mpc-ic-1-5]</code>
      * <code>[mpc][mpc-1-5]::[VERSION\_NUM][mpc-vn-1-5]</code>
  * The <code>[gmp][gmp-1-5]::[MPZ\_ROINIT\_N][gmp-zrn-1-5]</code> function is
    now `extern "C"`.

[gmp-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/index.html
[gmp-fs-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpf_sgn.html
[gmp-qd-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpq_denref.html
[gmp-qdc-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpq_denref_const.html
[gmp-qn-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpq_numref.html
[gmp-qnc-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpq_numref_const.html
[gmp-qs-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpq_sgn.html
[gmp-zep-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_even_p.html
[gmp-zfuip-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_fits_uint_p.html
[gmp-zfulp-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_fits_ulong_p.html
[gmp-zfusp-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_fits_ushort_p.html
[gmp-zg-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_getlimbn.html
[gmp-zop-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_odd_p.html
[gmp-zrn-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.MPZ_ROINIT_N.html
[gmp-zs-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_sgn.html
[gmp-zsi-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/gmp/fn.mpz_size.html
[mpc-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/index.html
[mpc-i-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.imagref.html
[mpc-i1-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.INEX1.html
[mpc-i2-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.INEX2.html
[mpc-ic-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.imagref_const.html
[mpc-ii-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.INEX_IM.html
[mpc-ir-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.INEX_RE.html
[mpc-r-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.realref.html
[mpc-rc-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.realref_const.html
[mpc-vn-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpc/fn.VERSION_NUM.html
[mpfr-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/index.html
[mpfr-cge-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.custom_get_exp.html
[mpfr-cgk-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.custom_get_kind.html
[mpfr-cgs-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.custom_get_size.html
[mpfr-cgsi-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.custom_get_significand.html
[mpfr-ci-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.custom_init.html
[mpfr-ge-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.get_exp.html
[mpfr-gp-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.get_prec.html
[mpfr-ip-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.inf_p.html
[mpfr-nap-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.nan_p.html
[mpfr-nup-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.number_p.html
[mpfr-rp-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.regular_p.html
[mpfr-s-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.signbit.html
[mpfr-vn-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.VERSION_NUM.html
[mpfr-zp-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/mpfr/fn.zero_p.html

Version 1.4.13 (2023-02-28)
===========================

  * The commits [`66d78412f..53da44400`] and [`3d3e2d9b0..5a1b1f9a2`] from the [4.1
    branch][mpfr 4.1 branch] of [MPFR] were merged.

[`3d3e2d9b0..5a1b1f9a2`]: https://gitlab.inria.fr/mpfr/mpfr/-/compare/3d3e2d9b0...5a1b1f9a2
[`66d78412f..53da44400`]: https://gitlab.inria.fr/mpfr/mpfr/-/compare/66d78412f...53da44400
[mpfr 4.1 branch]: https://gitlab.inria.fr/mpfr/mpfr/-/tree/4.1

Version 1.4.12 (2022-12-11)
===========================

  * [MPFR] was updated from version 4.1.1 to 4.1.1-p1.
  * Bug fix: compilation fixed for new git repositories without HEAD, such as
    for a new cargo project ([issue 27]).

[issue 27]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/27

Version 1.4.11 (2022-11-17)
===========================

  * [MPFR] was updated from version 4.1.0-p13 to 4.1.1.

Version 1.4.10 (2022-08-06)
===========================

  * All the changesets until [`402b9c4efacb`] from the [gmp-6.2 branch] of [GMP]
    were merged.
  * [MPFR] is now configured with `--disable-decimal-float --disable-float128`
    which avoids some failed tests with Clang in some situations.
  * Build caching should now work properly when the `CFLAGS` environment
    variable is set during compilation.

[`402b9c4efacb`]: https://gmplib.org/repo/gmp-6.2/rev/402b9c4efacb

Version 1.4.9 (2022-07-20)
==========================

  * Link system libraries statically in case of static musl target ([merge
    request 2]).
  * The [`f4ff6ff711ed` changeset] from the [gmp-6.2 branch] of [GMP] was merged
    to avoid the `x18` register for arm64 since it is reserved in Darwin ([issue
    25]).

[`f4ff6ff711ed` changeset]: https://gmplib.org/repo/gmp-6.2/rev/f4ff6ff711ed
[gmp-6.2 branch]: https://gmplib.org/repo/gmp-6.2
[issue 25]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/25
[merge request 2]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/merge_requests/2

Version 1.4.8 (2022-04-12)
==========================

  * Bug fix: <code>[mpfr][mpfr-1-4]::[rootn\_ui][mpfr-ru-1-4]</code> was linking
    to the MPFR function `mpfr_root` instead of `mpfr_rootn_ui`.

Version 1.4.7 (2021-09-23)
==========================

  * Bug fix: prefer `CC` environment variable over `gcc` when probing system
    libraries for the `use-system-libs` feature ([issue 20]).

[issue 20]: https://gitlab.com/tspiteri/gmp-mpfr-sys/-/issues/20

Version 1.4.6 (2021-07-29)
==========================

  * [MPFR] was updated from version 4.1.0-p12 to 4.1.0-p13.

Version 1.4.5 (2021-04-23)
==========================

  * [MPFR] was updated from version 4.1.0-p11 to 4.1.0-p12.

Version 1.4.4 (2021-03-25)
==========================

  * [MPFR] was updated from version 4.1.0-p7 to 4.1.0-p11.

Version 1.4.3 (2021-02-14)
==========================

  * [MPFR] was updated from version 4.1.0 to 4.1.0-p7.
  * The [`GMP_MPFR_SYS_CACHE`][cache-1-4] environment variable can now
    also be set to a single underscore (`"_"`) to disable caching, in
    case setting it to an empty string is not possible in some system.

Version 1.4.2 (2020-11-18)
==========================

  * [GMP] was updated from version 6.2.0 to 6.2.1.

Version 1.4.1 (2020-10-25)
==========================

  * [MPC] was updated from version 1.2.0 to 1.2.1.
  * The [`c-no-tests`][feat-exp-1-4] experimental feature was added.

Version 1.4.0 (2020-09-02)
==========================

  * [MPC] was updated from version 1.1.0 to 1.2.0.

[cache-1-4]: https://docs.rs/gmp-mpfr-sys/~1.4/gmp_mpfr_sys/#caching-the-built-c-libraries
[feat-exp-1-4]: https://docs.rs/gmp-mpfr-sys/~1.4/gmp_mpfr_sys/#experimental-optional-features
[mpfr-1-4]: https://docs.rs/gmp-mpfr-sys/~1.4/gmp_mpfr_sys/mpfr/index.html
[mpfr-ru-1-4]: https://docs.rs/gmp-mpfr-sys/~1.4/gmp_mpfr_sys/mpfr/fn.rootn_ui.html

Version 1.3.1 (2020-07-17)
==========================

  * Fixed MSYS2 build for rustc 1.46.0.

Version 1.3.0 (2020-07-13)
==========================

  * [MPFR] was updated from version 4.0.2-p9 to 4.1.0.
  * The internal details of
    <code>[gmp][gmp-1-3]::[mpz\_t][gmp-mpz-1-3]</code>,
    <code>[gmp][gmp-1-3]::[mpf\_t][gmp-mpf-1-3]</code>,
    <code>[gmp][gmp-1-3]::[randseed\_t][gmp-randseed-1-3]</code> and
    <code>[mpfr][mpfr-1-3]::[mpfr\_t][mpfr-mpfr-1-3]</code> have been
    changed to use [`NonNull`] instead of [mutable pointers][pointer].
  * The internal details of
    <code>[gmp][gmp-1-3]::[randfnptr\_t][gmp-randfnptr-1-3]</code>
    have been changed to reflect that its functions are not nullable.
  * Cross compilation will now fail if the experimental feature
    `force-cross` is not enabled, because cross compilation is not
    tested or supported and can lead to silent failures that are hard
    to debug, especially if this crate is an indirect dependency.

[gmp-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/gmp/index.html
[gmp-mpf-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/gmp/struct.mpf_t.html
[gmp-mpz-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/gmp/struct.mpz_t.html
[gmp-randfnptr-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/gmp/struct.randfnptr_t.html
[gmp-randseed-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/gmp/struct.randseed_t.html
[mpfr-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/mpfr/index.html
[mpfr-mpfr-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/mpfr/struct.mpfr_t.html

Version 1.2.4 (2020-06-14)
==========================

  * [MPFR] was updated from version 4.0.2-p7 to 4.0.2-p9.

Version 1.2.3 (2020-06-01)
==========================

  * The experimental feature `force-cross` was added. It is ignored in
    version 1.2 but will be required for cross compilation attempts
    from version 1.3, because cross compilation is not tested or
    supported and can lead to silent failures that are hard to debug,
    especially if this crate is an indirect dependency.
  * Bug fix: cross-compilation C libraries were being cached in the
    same directory as native C libraries.

Version 1.2.2 (2020-04-08)
==========================

  * [MPFR] was updated from version 4.0.2-p1 to 4.0.2-p7.
  * The missing function
    <code>[gmp][gmp-1-2]::[MPZ\_ROINIT\_N][gmp-mrn-1-2]</code> was
    added.
  * The missing macro [`MPFR_DECL_INIT`][mpfr-di-1-2] was added.

Version 1.2.1 (2020-03-13)
==========================

  * Some improvements were made to enable compilation or cross
    compilation on more platforms. While such platforms are not tested
    automatically and may not work, merge requests that improve the
    situation are accepted.

Version 1.2.0 (2020-01-18)
==========================

  * The crate now requires rustc version 1.37.0 or later.
  * The crate now supports [`no_std`].
  * [GMP] was updated from version 6.1.2 to 6.2.0.
  * The implementation details of
    <code>[gmp][gmp-1-2]::[randstate\_t][gmp-rs-1-2]</code> have been
    changed to reflect that [GMP] can leave some fields unused and
    uninitialized.
  * The experimental feature `use-system-libs` was added.

[gmp-1-2]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/index.html
[gmp-mrn-1-2]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/fn.MPZ_ROINIT_N.html
[gmp-rs-1-2]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/struct.randstate_t.html
[mpfr-di-1-2]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/macro.MPFR_DECL_INIT.html

Version 1.1.14 (2019-07-04)
===========================

  * [MPFR] was updated from version 4.0.2 to 4.0.2-p1.
  * The documentation examples now use [`MaybeUninit`] instead of
    <code>[mem][`mem`]::[uninitialized][`uninitialized`]</code>.

Version 1.1.13 (2019-05-17)
===========================

  * Expose the internal fields of
    <code>[gmp][gmp-1-1]::[mpq\_t][gmp-mpq-1-1]</code>,
    <code>[gmp][gmp-1-1]::[randstate\_t][gmp-rs-1-1]</code> and
    <code>[mpc][mpc-1-1]::[mpc\_t][mpc-mpc-1-1]</code>, and add some
    notes about compatibility in the documentation.

Version 1.1.12 (2019-03-08)
===========================

  * The built C libraries are now cached.

Version 1.1.11 (2019-02-01)
===========================

  * Update [MPFR] 4.0.1-p14 -> 4.0.2.

Version 1.1.10 (2019-01-04)
===========================

  * Update [MPFR] 4.0.1-p13 -> 4.0.1-p14.
  * During Windows build use
    <code>[std][`std`]::[os][`os`]::[windows][`windows`]::[fs][`fs`]::[symlink\_dir][`symlink_dir`]</code>
    to save on some copying if allowed (Windows 1703+ developer mode).

Version 1.1.9 (2018-10-05)
==========================

  * Update [MPFR] 4.0.1-p11 -> 4.0.1-p13.
  * Fix function parameters that should be
    [`intmax_t`][libc-intmax-0-2] or [`uintmax_t`][libc-uintmax-0-2].

Version 1.1.8 (2018-07-23)
==========================

  * Update [MPFR] 4.0.1-p9 -> 4.0.1-p11.

Version 1.1.7 (2018-07-11)
==========================

  * Update [MPFR] 4.0.1-p6 -> 4.0.1-p9.

Version 1.1.6 (2018-05-29)
==========================

  * Automatically work around [Rust issue 47048][rust-47048].

Version 1.1.5 (2018-05-02)
==========================

  * Update [MPFR] 4.0.1 -> 4.0.1-p6.

Version 1.1.4 (2018-04-23)
==========================

  * Add missing [GMP], [MPFR] and [MPC] functions that take a
    <code>[\*mut][pointer] [FILE][libc-FILE-0-2]</code> argument.

Version 1.1.3 (2018-04-05)
==========================

  * Fix linkage of [MPFR] `uj` and `sj` functions.

Version 1.1.2 (2018-04-05)
==========================

  * Add missing [MPFR] and [MPC] functions with `uj` and `sj`, using
    [`c_ulonglong`] and [`c_longlong`] respectively.
  * Add missing [MPFR] [`dump`][mpfr-dump-1-1] function.

Version 1.1.1 (2018-02-09)
==========================

  * Update [MPFR] 4.0.0 -> 4.0.1.
  * Fix the type of the `tab` parameter of
    <code>[mpfr][mpfr-1-1]::[sum][mpfr-sum-1-1]</code> to
    <code>[\*const][pointer] [\*mut][pointer] [mpfr\_t][mpfr-mpfr-1-1]</code>
    instead of
    <code>[\*mut][pointer] [\*mut][pointer] [mpfr\_t][mpfr-mpfr-1-1]</code>.
  * Document the `DEP_GMP_LIMB_BITS` build script metadata.
  * Add `DEP_GMP_OUT_DIR`, `DEP_GMP_LIB_DIR`, and
    `DEP_GMP_INCLUDE_DIR` build script metadata.

Version 1.1.0 (2018-01-12)
==========================

  * Update [MPFR] 3.1.6-p1 -> 4.0.0.
  * Update [MPC] 1.0.3 -> 1.1.0.
  * Deprecate and hide documentation for
    <code>[mpfr][mpfr-1-1]::[rnd\_t][mpfr-rnd-1-1]::RNDNA</code>;
    `MPFR_RNDNA` is not documented by [MPFR], and *mpfr.h* says it
    should not be used.
  * Use [`c_int`] instead of
    <code>[#\[repr(C)\]][repr-C] [enum]</code> for the private
    enumerated type inside
    <code>[#\[repr(C)\]][repr-C] [struct] [randstate_t][gmp-rs-1-1]</code>.

[gmp-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/index.html
[gmp-mpq-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/struct.mpq_t.html
[gmp-rs-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/struct.randstate_t.html
[libc-FILE-0-2]: https://docs.rs/libc/~0.2/libc/enum.FILE.html
[libc-intmax-0-2]: https://docs.rs/libc/~0.2/libc/type.intmax_t.html
[libc-uintmax-0-2]: https://docs.rs/libc/~0.2/libc/type.uintmax_t.html
[mpc-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/index.html
[mpc-mpc-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/struct.mpc_t.html
[mpfr-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/index.html
[mpfr-dump-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.dump.html
[mpfr-mpfr-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/struct.mpfr_t.html
[mpfr-rnd-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/enum.rnd_t.html
[mpfr-sum-1-1]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.sum.html

Version 1.0.8 (2017-11-08)
==========================

  * Update [MPFR] 3.1.6 -> 3.1.6-p1.

Version 1.0.7 (2017-09-10)
==========================

  * Update [MPFR] 3.1.5-p9 -> 3.1.6.

Version 1.0.6 (2017-07-24)
==========================

  * Update [MPFR] 3.1.5-p8 -> 3.1.5-p9.

Version 1.0.5 (2017-06-26)
==========================

  * Bug fix: use C linkage for inline functions.

Version 1.0.4 (2017-06-20)
==========================

  * Update [MPFR] 3.1.5 -> 3.1.5-p8.

Version 1.0.3 (2017-06-06)
==========================

  * Add <code>[gmp][gmp-1-0]::[mpq\_numref\_const][gmp-mnc-1-0]</code>,
    <code>[gmp][gmp-1-0]::[mpq\_denref\_const][gmp-mdc-1-0]</code>,
    <code>[mpc][mpc-1-0]::[realref\_const][mpc-rc-1-0]</code> and
    <code>[mpc][mpc-1-0]::[imagref\_const][mpc-ic-1-0]</code>.
  * Add inline version of functions which are inline in *gmp.h* and
    *mpfr.h*.
  * Bug fix: <code>[gmp][gmp-1-0]::[mpz\_even\_p][gmp-mep-1-0]</code>.

Version 1.0.2 (2017-05-20)
==========================

  * Add features `mpfr` and `mpc`, which are enabled by default, to
    allow opting out of the [MPFR] and [MPC] libraries.

Version 1.0.1 (2017-05-06)
==========================

  * Expliciltly link to *gcc\_eh* and *pthread* under MinGW.

Version 1.0.0 (2017-04-24)
==========================

  * [GMP] 6.1.2, [MPFR] 3.1.5, [MPC] 1.0.3

[gmp-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/gmp/index.html
[gmp-mdc-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/gmp/fn.mpq_denref_const.html
[gmp-mep-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/gmp/fn.mpz_even_p.html
[gmp-mnc-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/gmp/fn.mpq_numref_const.html
[mpc-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/mpc/index.html
[mpc-ic-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/mpc/fn.imagref_const.html
[mpc-rc-1-0]: https://docs.rs/gmp-mpfr-sys/~1.0/gmp_mpfr_sys/mpc/fn.realref_const.html

[GMP]: https://gmplib.org/
[MPC]: https://www.multiprecision.org/
[MPFR]: https://www.mpfr.org/
[`MaybeUninit`]: https://doc.rust-lang.org/nightly/core/mem/union.MaybeUninit.html
[`NonNull`]: https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html
[`c_int`]: https://doc.rust-lang.org/std/os/raw/type.c_int.html
[`c_longlong`]: https://doc.rust-lang.org/std/os/raw/type.c_longlong.html
[`c_ulonglong`]: https://doc.rust-lang.org/std/os/raw/type.c_ulonglong.html
[`fs`]: https://doc.rust-lang.org/nightly/std/os/windows/fs/index.html
[`mem`]: https://doc.rust-lang.org/nightly/core/mem/index.html
[`no_std`]: https://doc.rust-lang.org/nightly/embedded-book/intro/no-std.html
[`os`]: https://doc.rust-lang.org/nightly/std/os/index.html
[`std`]: https://doc.rust-lang.org/nightly/std/index.html
[`symlink_dir`]: https://doc.rust-lang.org/nightly/std/os/windows/fs/fn.symlink_dir.html
[`uninitialized`]: https://doc.rust-lang.org/nightly/core/mem/fn.uninitialized.html
[`windows`]: https://doc.rust-lang.org/nightly/std/os/windows/index.html
[enum]: https://doc.rust-lang.org/nightly/reference/types/enum.html
[pointer]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
[repr-C]: https://doc.rust-lang.org/nightly/reference/type-layout.html#the-c-representation
[rust-47048]: https://github.com/rust-lang/rust/issues/47048
[struct]: https://doc.rust-lang.org/nightly/reference/types/struct.html
