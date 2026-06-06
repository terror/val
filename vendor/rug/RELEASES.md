<!-- Copyright © 2016–2026 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without modification, are
permitted in any medium without royalty provided the copyright notice and this
notice are preserved. This file is offered as-is, without any warranty. -->

Version 1.30.0 (2026-04-27)
===========================

  * Arithmetic operators are now overloaded to work between:
     * [`Integer`][int-1-30] and [`MiniInteger`][mi-1-30]
     * [`Rational`][rat-1-30] and [`MiniRational`][mr-1-30]
     * [`Float`][flo-1-30] and [`MiniFloat`][mf-1-30]
     * [`Complex`][com-1-30] and [`MiniComplex`][mc-1-30]
  * The following method was added to [`MiniRational`][mr-1-30]:
     * [`const_from_canonical`][mr-cfc-1-30]

[com-1-30]: https://docs.rs/rug/~1.30/rug/struct.Complex.html
[flo-1-30]: https://docs.rs/rug/~1.30/rug/struct.Float.html
[int-1-30]: https://docs.rs/rug/~1.30/rug/struct.Integer.html
[mc-1-30]: https://docs.rs/rug/~1.30/rug/complex/struct.MiniComplex.html
[mf-1-30]: https://docs.rs/rug/~1.30/rug/float/struct.MiniFloat.html
[mi-1-30]: https://docs.rs/rug/~1.30/rug/integer/struct.MiniInteger.html
[mr-1-30]: https://docs.rs/rug/~1.30/rug/rational/struct.MiniRational.html
[mr-cfc-1-30]: https://docs.rs/rug/~1.30/rug/rational/struct.MiniRational.html#method.const_from_canonical
[rat-1-30]: https://docs.rs/rug/~1.30/rug/struct.Rational.html

Version 1.29.0 (2026-03-20)
===========================

  * The [*az* crate] dependency was updated to [version 1.3][az-1-3].
  * The [gmp-mpfr-sys] dependency was updated to [version 1.7][sys-1-7].
  * The following methods were added to [`Complex`][com-1-29]:
     * [`exp2`][com-e2-1-29], [`exp2_mut`][com-e2m-1-29],
       [`exp2_round`][com-e2ro-1-29], [`exp2_ref`][com-e2re-1-29]
     * [`exp10`][com-e10-1-29], [`exp10_mut`][com-e10m-1-29],
       [`exp10_round`][com-e10ro-1-29], [`exp10_ref`][com-e10re-1-29]
     * [`log2`][com-l2-1-29], [`log2_mut`][com-l2m-1-29],
       [`log2_round`][com-l2ro-1-29], [`log2_ref`][com-l2re-1-29]

[az-1-3]: https://docs.rs/az/~1.3/az/index.html
[com-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html
[com-e10-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp10
[com-e10m-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp10_mut
[com-e10re-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp10_ref
[com-e10ro-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp10_round
[com-e2-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp2
[com-e2m-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp2_mut
[com-e2re-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp2_ref
[com-e2ro-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.exp2_round
[com-l2-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.log2
[com-l2m-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.log2_mut
[com-l2re-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.log2_ref
[com-l2ro-1-29]: https://docs.rs/rug/~1.29/rug/struct.Complex.html#method.log2_round
[sys-1-7]: https://docs.rs/gmp-mpfr-sys/~1.7/gmp_mpfr_sys/index.html

Version 1.28.1 (2026-01-21)
===========================

  * Bug fix: incorrect underflow when dividing [`Rational`][rat-1-28] by
    [`Float`][flo-1-28] ([issue 85]).

[issue 85]: https://gitlab.com/tspiteri/rug/-/issues/85
[rat-1-28]: https://docs.rs/rug/~1.28/rug/struct.Rational.html

Version 1.28.0 (2025-08-21)
===========================

  * The crate now requires rustc version 1.85.0 or later.
  * The following methods were added to [`Float`][flo-1-28]:
     * [`as_shl`][flo-ashl-1-28], [`as_shr`][flo-ashr-1-28]
  * The following methods were added to [`Complex`][com-1-28]:
     * [`as_shl`][com-ashl-1-28], [`as_shr`][com-ashr-1-28]
  * The experimental feature [`num-complex`][feat-exp-1-28] was added ([issue
    84]).
  * The experimental feature [`borsh`][feat-exp-1-28] was added ([merge request
    6]).

[com-1-28]: https://docs.rs/rug/~1.28/rug/struct.Complex.html
[com-ashl-1-28]: https://docs.rs/rug/~1.28/rug/struct.Complex.html#method.as_shl
[com-ashr-1-28]: https://docs.rs/rug/~1.28/rug/struct.Complex.html#method.as_shr
[feat-exp-1-28]: https://docs.rs/rug/~1.28/rug/index.html#experimental-optional-features
[flo-1-28]: https://docs.rs/rug/~1.28/rug/struct.Float.html
[flo-ashl-1-28]: https://docs.rs/rug/~1.28/rug/struct.Float.html#method.as_shl
[flo-ashr-1-28]: https://docs.rs/rug/~1.28/rug/struct.Float.html#method.as_shr
[issue 84]: https://gitlab.com/tspiteri/rug/-/issues/84
[merge request 6]: https://gitlab.com/tspiteri/rug/-/merge_requests/6

Version 1.27.0 (2025-01-24)
===========================

  * The [*az* crate], which is a dependency, is now re-exported ([issue 73]).
  * The following methods were added to [`Float`][flo-1-27]:
     * [`to_integer_in_place`][flo-tiip-1-27],
       [`to_integer_round_in_place`][flo-tirip-1-27],
       [`to_integer_exp_in_place`][flo-tieip-1-27] ([issue 75])
     * [`to_rational_in_place`][flo-trip-1-27] ([issue 75])
     * [`remainder_quo31`][flo-rq-1-27], [`remainder_quo31_mut`][flo-rqm-1-27],
       [`remainder_quo31_round`][flo-rqr-1-27] ([issue 76])
     * [`remainder_quo31_from`][flo-rqf-1-27],
       [`remainder_quo31_from_round`][flo-rqfr-1-27]
     * [`remainder_quo31_ref`][flo-rqref-1-27]
     * [`frexp`][flo-f-1-27], [`frexp_mut`][flo-fm-1-27],
       [`frexp_ref`][flo-fr-1-27] ([issue 76])
  * Bug fix: <code>[Float][flo-1-27]::[to\_f16][flo-tf16-1-27]</code> and
    <code>[Float][flo-1-27]::[to\_f128][flo-tf128-1-27]</code> were giving bad
    results for some subnormal numbers ([issue 80]).

Compatibility note
------------------

  * These subnormalization methods for [`Float`][flo-1-27] now handle numbers with
    exponents smaller than subnormals by rounding them to either 0 or a
    subnormal with the minimum nonzero magnitude ([issue 78]):
      * [`subnormalize_ieee`][flo-si-1-27],
        [`subnormalize_ieee_round`][flo-sir-1-27]
      * [`subnormalize`][flo-s-1-27],
        [`subnormalize_round`][flo-sr-1-27]

[flo-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html
[flo-f-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.frexp
[flo-fm-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.frexp_mut
[flo-fr-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.frexp_ref
[flo-rq-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.remainder_quo31
[flo-rqf-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.remainder_quo31_from
[flo-rqfr-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.remainder_quo31_from_round
[flo-rqm-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.remainder_quo31_mut
[flo-rqr-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.remainder_quo31_round
[flo-rqref-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.remainder_quo31_ref
[flo-s-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.subnormalize
[flo-si-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.subnormalize_ieee
[flo-sir-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.subnormalize_ieee_round
[flo-sr-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.subnormalize_round
[flo-tf128-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.to_f128
[flo-tf16-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.to_f16
[flo-tieip-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.to_integer_exp_in_place
[flo-tiip-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.to_integer_in_place
[flo-tirip-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.to_integer_round_in_place
[flo-trip-1-27]: https://docs.rs/rug/~1.27/rug/struct.Float.html#method.to_rational_in_place
[issue 73]: https://gitlab.com/tspiteri/rug/-/issues/73
[issue 75]: https://gitlab.com/tspiteri/rug/-/issues/75
[issue 76]: https://gitlab.com/tspiteri/rug/-/issues/76
[issue 78]: https://gitlab.com/tspiteri/rug/-/issues/78
[issue 80]: https://gitlab.com/tspiteri/rug/-/issues/80

Version 1.26.1 (2024-09-11)
===========================

  * Bug fix: implementations of [`CompleteRound`][compr-1-26] were wrongly using
    [`prec_t`][sys-mpfr-p-1-6] instead of [`u32`] for
    <code>[CompleteRound][compr-1-26]::[Prec][compr-p-1-26]</code> ([issue 72]).

[compr-1-26]: https://docs.rs/rug/~1.26/rug/ops/trait.CompleteRound.html
[compr-p-1-26]: https://docs.rs/rug/~1.26/rug/ops/trait.CompleteRound.html#associatedtype.Prec
[issue 72]: https://gitlab.com/tspiteri/rug/-/issues/72
[sys-mpfr-p-1-6]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/mpfr/type.prec_t.html

Version 1.26.0 (2024-08-30)
===========================

  * Bug fix: [`Complete`][c-1-26] was not implemented but documented for
    incomplete values produced by the following methods ([issue 69]):
      * <code>[Integer][int-1-26]::[clamp\_ref][int-cr-1-26]</code>,
        <code>[Integer][int-1-26]::[invert\_ref][int-ir-1-26]</code>,
        <code>[Integer][int-1-26]::[pow\_mod\_ref][int-pmr-1-26]</code>
      * <code>[Integer][int-1-26]::[random\_bits][int-rb-1-26]</code>,
        <code>[Integer][int-1-26]::[random\_below\_ref][int-rbr-1-26]</code>
  * <code>[Float][flo-1-26]::[get\_significand][flo-gs-1-26]</code> is now
    usable in const context.
  * The experimental feature [`nightly-float`][feat-exp-1-26] was added ([issue
    68]).

[c-1-26]: https://docs.rs/rug/~1.26/rug/trait.Complete.html
[feat-exp-1-26]: https://docs.rs/rug/~1.26/rug/index.html#experimental-optional-features
[flo-1-26]: https://docs.rs/rug/~1.26/rug/struct.Float.html
[flo-gs-1-26]: https://docs.rs/rug/~1.26/rug/struct.Float.html#method.get_significand
[int-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html
[int-cr-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.clamp_ref
[int-ir-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.invert_ref
[int-pmr-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.pow_mod_ref
[int-rb-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.random_bits
[int-rbr-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.random_below_ref
[issue 68]: https://gitlab.com/tspiteri/rug/-/issues/68
[issue 69]: https://gitlab.com/tspiteri/rug/-/issues/69

Version 1.25.1 (2024-09-11)
===========================

  * Bug fix: implementations of [`CompleteRound`][compr-1-25] were wrongly using
    [`prec_t`][sys-mpfr-p-1-6] instead of [`u32`] for
    <code>[CompleteRound][compr-1-25]::[Prec][compr-p-1-25]</code> ([issue 72]).

[compr-1-25]: https://docs.rs/rug/~1.25/rug/ops/trait.CompleteRound.html
[compr-p-1-25]: https://docs.rs/rug/~1.25/rug/ops/trait.CompleteRound.html#associatedtype.Prec
[issue 72]: https://gitlab.com/tspiteri/rug/-/issues/72
[sys-mpfr-p-1-6]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/mpfr/type.prec_t.html

Version 1.25.0 (2024-07-18)
===========================

  * The following functions were added:
     * <code>[float][flom-1-25]::[prec\_min\_64][flom-pmin-1-25]</code>,
       <code>[float][flom-1-25]::[prec\_max\_64][flom-pmax-1-25]</code>
  * The <code>[complex][comm-1-25]::[Prec64][comm-p-1-25]</code> trait was
    added.
  * The following methods were added:
     * <code>[Float][flo-1-25]::[new\_64][flo-n-1-25]</code>,
       <code>[Float][flo-1-25]::[with\_val\_64][flo-wv-1-25]</code>,
       <code>[Float][flo-1-25]::[with\_val\_round\_64][flo-wvr-1-25]</code>
     * <code>[Float][flo-1-25]::[prec\_64][flo-p-1-25]</code>,
       <code>[Float][flo-1-25]::[set\_prec\_64][flo-sp-1-25]</code>,
       <code>[Float][flo-1-25]::[set\_prec\_round\_64][flo-spr-1-25]</code>
     * <code>[Complex][com-1-25]::[new\_64][com-n-1-25]</code>,
       <code>[Complex][com-1-25]::[with\_val\_64][com-wv-1-25]</code>,
       <code>[Complex][com-1-25]::[with\_val\_round\_64][com-wvr-1-25]</code>
     * <code>[Complex][com-1-25]::[prec\_64][com-p-1-25]</code>,
       <code>[Complex][com-1-25]::[set\_prec\_64][com-sp-1-25]</code>,
       <code>[Complex][com-1-25]::[set\_prec\_round\_64][com-spr-1-25]</code>

[com-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html
[com-n-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html#method.new_64
[com-p-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html#method.prec_64
[com-sp-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html#method.set_prec_64
[com-spr-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html#method.set_prec_round_64
[com-wv-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html#method.with_val_64
[com-wvr-1-25]: https://docs.rs/rug/~1.25/rug/struct.Complex.html#method.with_val_round_64
[comm-1-25]: https://docs.rs/rug/~1.25/rug/complex/index.html
[comm-p-1-25]: https://docs.rs/rug/~1.25/rug/complex/trait.Prec64.html
[flo-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html
[flo-n-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html#method.new_64
[flo-p-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html#method.prec_64
[flo-sp-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html#method.set_prec_64
[flo-spr-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html#method.set_prec_round_64
[flo-wv-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html#method.with_val_64
[flo-wvr-1-25]: https://docs.rs/rug/~1.25/rug/struct.Float.html#method.with_val_round_64
[flom-1-25]: https://docs.rs/rug/~1.25/rug/float/index.html
[flom-pmax-1-25]: https://docs.rs/rug/~1.25/rug/float/fn.prec_max_64.html
[flom-pmin-1-25]: https://docs.rs/rug/~1.25/rug/float/fn.prec_min_64.html

Version 1.24.1 (2024-04-01)
===========================

  * Bug fix: a precondition for `slice::from_raw_parts` was being violated
    ([issue 63]).

[issue 63]: https://gitlab.com/tspiteri/rug/-/issues/63

Version 1.24.0 (2024-01-25)
===========================

  * [`Float`][flo-1-24] now implements
    <code>[AssignRound][assr-1-24]\<[bool][`bool`]></code>.
  * The following methods are now usable in const context:
      * <code>[Float][flo-1-24]::[as\_neg][flo-an-1-24]</code>,
        <code>[Float][flo-1-24]::[as\_abs][flo-aa-1-24]</code>
      * <code>[Complex][com-1-24]::[as\_neg][com-an-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_conj][com-ac-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_mul\_i][com-am-1-24]</code>
      * <code>[MiniInteger][mi-1-24]::[borrow][mi-b-1-24]</code>
      * <code>[MiniRational][mr-1-24]::[borrow][mr-b-1-24]</code>
      * <code>[MiniFloat][mf-1-24]::[borrow][mf-b-1-24]</code>
      * <code>[MiniComplex][mc-1-24]::[borrow][mc-b-1-24]</code>
  * The `std` [optional feature][feat-1-24] was added ([issue 62]). The feature
    is enabled by default, and the crate will be compiled without the standard
    library (`no_std`) if the feature is disabled.
  * [`MiniRational`][mr-1-24] now implements
      * <code>[From][`From`]\<[MiniInteger][mi-1-24]></code>,
        <code>[Assign][ass-1-24]\<[MiniInteger][mi-1-24]></code>
  * [`MiniFloat`][mf-1-24] now implements
      * <code>[From][`From`]\<[bool][`bool`]></code>,
        <code>[Assign][ass-1-24]\<[bool][`bool`]></code>
  * [`MiniComplex`][mc-1-24] now implements
      * <code>[From][`From`]\<[MiniFloat][mf-1-24]></code>,
        <code>[Assign][ass-1-24]\<[MiniFloat][mf-1-24]></code>
      * <code>[From][`From`]\<[(][tuple][MiniFloat][mf-1-24][,][tuple] [MiniFloat][mf-1-24][)][tuple]></code>,
        <code>[Assign][ass-1-24]\<[(][tuple][MiniFloat][mf-1-24][,][tuple] [MiniFloat][mf-1-24][)][tuple]></code>
  * The following methods were added to [`MiniInteger`][mi-1-24]:
      * [`const_from_bool`][mi-bool-1-24]
      * [`const_from_i8`][mi-i8-1-24], [`const_from_i16`][mi-i16-1-24],
        [`const_from_i32`][mi-i32-1-24], [`const_from_i64`][mi-i64-1-24],
        [`const_from_i128`][mi-i128-1-24], [`const_from_isize`][mi-isize-1-24]
      * [`const_from_u8`][mi-u8-1-24], [`const_from_u16`][mi-u16-1-24],
        [`const_from_u32`][mi-u32-1-24], [`const_from_u64`][mi-u64-1-24],
        [`const_from_u128`][mi-u128-1-24], [`const_from_usize`][mi-usize-1-24]
  * The following method was added to [`MiniRational`][mr-1-24]:
      * [`const_from_integer`][mr-cfi-1-24]
  * The following methods were added to [`MiniFloat`][mf-1-24]:
      * [`const_from_bool`][mf-bool-1-24]
      * [`const_from_i8`][mf-i8-1-24], [`const_from_i16`][mf-i16-1-24],
        [`const_from_i32`][mf-i32-1-24], [`const_from_i64`][mf-i64-1-24],
        [`const_from_i128`][mf-i128-1-24], [`const_from_isize`][mf-isize-1-24]
      * [`const_from_u8`][mf-u8-1-24], [`const_from_u16`][mf-u16-1-24],
        [`const_from_u32`][mf-u32-1-24], [`const_from_u64`][mf-u64-1-24],
        [`const_from_u128`][mf-u128-1-24], [`const_from_usize`][mf-usize-1-24]
      * [`const_from_f32`][mf-f32-1-24], [`const_from_f64`][mf-f64-1-24]
      * [`const_from_special`][mf-cfs-1-24]
  * The following methods were added to [`MiniComplex`][mc-1-24]:
      * [`const_from_real`][mc-cfr-1-24], [`const_from_parts`][mc-cfp-1-24]
  * When the `num-traits` [experimental feature][feat-exp-1-24] is enabled,
    the following traits are implemented for [`Integer`][int-1-24]:
      * [`CheckedDiv`][nt-0-2-cd], [`CheckedRem`][nt-0-2-cr]
      * [`Euclid`][nt-0-2-e], [`CheckedEuclid`][nt-0-2-ce]

Compatibility notes
-------------------

  * The following methods no longer set the [MPFR NaN flag][sys-mpfr-sn-1-6] if
    a NaN is encountered:
      * <code>[Float][flo-1-24]::[as\_neg][flo-an-1-24]</code>,
        <code>[Float][flo-1-24]::[as\_abs][flo-aa-1-24]</code>
      * <code>[Complex][com-1-24]::[as\_neg][com-an-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_conj][com-ac-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_mul\_i][com-am-1-24]</code>
  * Functionality that depends on `std` now requires the `std` feature to be
    enabled. While the `std` feature is enabled by default, it is now disabled
    if the crate’s default features are disabled in [*Cargo.toml*]. In this
    case, `std` must be added to the list of features to reenable the
    functionality.
  * The implementation of the deprecated [`SmallInteger`][smi-1-24],
    [`SmallRational`][smr-1-24], [`SmallFloat`][smf-1-24] and
    [`SmallComplex`][smc-1-24] was changed to remove [their soundness
    issue][issue 52]. They now allocate memory for the digits, though their
    whole point was to be non-allocating.

[ass-1-24]: https://docs.rs/rug/~1.24/rug/trait.Assign.html
[assr-1-24]:  https://docs.rs/rug/~1.24/rug/ops/trait.AssignRound.html
[com-1-24]: https://docs.rs/rug/~1.24/rug/struct.Complex.html
[com-ac-1-24]: https://docs.rs/rug/~1.24/rug/struct.Complex.html#method.as_conj
[com-am-1-24]: https://docs.rs/rug/~1.24/rug/struct.Complex.html#method.as_mul_i
[com-an-1-24]: https://docs.rs/rug/~1.24/rug/struct.Complex.html#method.as_neg
[feat-1-24]: https://docs.rs/rug/~1.24/rug/index.html#optional-features
[feat-exp-1-24]: https://docs.rs/rug/~1.24/rug/index.html#experimental-optional-features
[flo-1-24]: https://docs.rs/rug/~1.24/rug/struct.Float.html
[flo-aa-1-24]: https://docs.rs/rug/~1.24/rug/struct.Float.html#method.as_abs
[flo-an-1-24]: https://docs.rs/rug/~1.24/rug/struct.Float.html#method.as_neg
[int-1-24]: https://docs.rs/rug/~1.24/rug/struct.Integer.html
[issue 62]: https://gitlab.com/tspiteri/rug/-/issues/62
[mc-1-24]: https://docs.rs/rug/~1.24/rug/complex/struct.MiniComplex.html
[mc-b-1-24]: https://docs.rs/rug/~1.24/rug/complex/struct.MiniComplex.html#method.borrow
[mc-cfp-1-24]: https://docs.rs/rug/~1.24/rug/complex/struct.MiniComplex.html#method.const_from_parts
[mc-cfr-1-24]: https://docs.rs/rug/~1.24/rug/complex/struct.MiniComplex.html#method.const_from_real
[mf-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html
[mf-b-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.borrow
[mf-bool-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_bool
[mf-cfs-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_special
[mf-f32-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_f32
[mf-f64-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_f64
[mf-i128-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_i128
[mf-i16-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_i16
[mf-i32-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_i32
[mf-i64-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_i64
[mf-i8-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_i8
[mf-isize-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_isize
[mf-u128-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_u128
[mf-u16-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_u16
[mf-u32-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_u32
[mf-u64-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_u64
[mf-u8-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_u8
[mf-usize-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.MiniFloat.html#method.const_from_usize
[mi-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html
[mi-b-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.borrow
[mi-bool-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_bool
[mi-i128-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_i128
[mi-i16-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_i16
[mi-i32-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_i32
[mi-i64-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_i64
[mi-i8-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_i8
[mi-isize-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_isize
[mi-u128-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_u128
[mi-u16-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_u16
[mi-u32-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_u32
[mi-u64-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_u64
[mi-u8-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_u8
[mi-usize-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.MiniInteger.html#method.const_from_usize
[mr-1-24]: https://docs.rs/rug/~1.24/rug/rational/struct.MiniRational.html
[mr-b-1-24]: https://docs.rs/rug/~1.24/rug/rational/struct.MiniRational.html#method.borrow
[mr-cfi-1-24]: https://docs.rs/rug/~1.24/rug/rational/struct.MiniRational.html#method.const_from_integer
[nt-0-2-cd]: https://docs.rs/num-traits/~0.2/num_traits/ops/checked/trait.CheckedDiv.html
[nt-0-2-ce]: https://docs.rs/num-traits/~0.2/num_traits/ops/euclid/trait.CheckedEuclid.html
[nt-0-2-cr]: https://docs.rs/num-traits/~0.2/num_traits/ops/checked/trait.CheckedRem.html
[nt-0-2-e]: https://docs.rs/num-traits/~0.2/num_traits/ops/euclid/trait.Euclid.html
[smc-1-24]: https://docs.rs/rug/~1.24/rug/complex/struct.SmallComplex.html
[smf-1-24]: https://docs.rs/rug/~1.24/rug/float/struct.SmallFloat.html
[smi-1-24]: https://docs.rs/rug/~1.24/rug/integer/struct.SmallInteger.html
[smr-1-24]: https://docs.rs/rug/~1.24/rug/rational/struct.SmallRational.html
[sys-mpfr-sn-1-6]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/mpfr/fn.set_nanflag.html

Version 1.23.0 (2024-01-18)
===========================

  * [`MiniInteger`][mi-1-23], [`MiniRational`][mr-1-23], [`MiniFloat`][mf-1-23]
    and [`MiniComplex`][mc-1-23] were added to replace
    [`SmallInteger`][smi-1-23], [`SmallRational`][smr-1-23],
    [`SmallFloat`][smf-1-23] and [`SmallComplex`][smc-1-23], which are now
    deprecated.
      * The new structs do *not* implement [`Deref`] directly, as this causes
        issues with the current Rust memory models ([issue 52]).
      * Borrowing requires the use of the [`borrow`][mi-b-1-23] methods, which
        return an object that implements [`Deref`].
      * The new [`borrow_excl`][mi-be-1-23] methods were also added; they
        require exclusive access in order to save on some housekeeping and to
        return references directly without the need of [`Deref`].
      * The new structs implement [`Copy`] and [`Sync`].

[issue 52]: https://gitlab.com/tspiteri/rug/-/issues/52
[mc-1-23]: https://docs.rs/rug/~1.23/rug/complex/struct.MiniComplex.html
[mf-1-23]: https://docs.rs/rug/~1.23/rug/float/struct.MiniFloat.html
[mi-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.MiniInteger.html
[mi-b-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.MiniInteger.html#method.borrow
[mi-be-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.MiniInteger.html#method.borrow_excl
[mr-1-23]: https://docs.rs/rug/~1.23/rug/rational/struct.MiniRational.html
[smc-1-23]: https://docs.rs/rug/~1.23/rug/complex/struct.SmallComplex.html
[smf-1-23]: https://docs.rs/rug/~1.23/rug/float/struct.SmallFloat.html
[smi-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.SmallInteger.html
[smr-1-23]: https://docs.rs/rug/~1.23/rug/rational/struct.SmallRational.html

Version 1.22.0 (2023-09-11)
===========================

  * Bug fix: implementations of [`PartialOrd`] and [`PartialEq`] between
    [`Rational`][rat-1-22] numbers and tuples of two primitive integers were
    breaking the transitivity property of the traits, so the implementations
    were removed ([issue 58]). See the compatibility note below.
  * Direct [`PartialOrd`] and [`PartialEq`] comparisons are now implemented
      * between [`Integer`][int-1-22] and [`SmallInteger`][smi-1-22],
      * between [`Rational`][rat-1-22] and [`SmallRational`][smr-1-22], and
      * between [`Float`][flo-1-22] and [`SmallFloat`][smf-1-22].
  * Direct [`PartialEq`] comparisons are now implemented between
    [`Complex`][com-1-22] and [`SmallComplex`][smc-1-22].
  * [`SmallInteger`][smi-1-22] can now be assigned or converted directly to
    [`Integer`][int-1-22] using [`Assign`][ass-1-22] and [`From`].
  * [`SmallRational`][smr-1-22] can now be assigned or converted directly to
    [`Rational`][rat-1-22] using [`Assign`][ass-1-22] and [`From`].
  * [`SmallFloat`][smf-1-22] can now be assigned directly to [`Float`][flo-1-22]
    using [`AssignRound`][assr-1-22] and [`Assign`][ass-1-22].
  * [`SmallComplex`][smc-1-22] can now be assigned directly to
    [`Complex`][com-1-22] using [`AssignRound`][assr-1-22] and
    [`Assign`][ass-1-22].
  * [`Debug`] is now implemented for [`SmallInteger`][smi-1-22],
    [`SmallRational`][smr-1-22], [`SmallFloat`][smf-1-22] and
    [`SmallComplex`][smc-1-22].
  * For the [`Complex`][com-1-22] struct, the method [`eq0`][com-e-1-22] was
    renamed to [`is_zero`][com-iz-1-22]. The old method name is deprecated.

Compatibility note
------------------

The implementations of [`PartialOrd`] and [`PartialEq`] between
[`Rational`][rat-1-22] numbers and tuples of two primitive integers were
breaking the transitivity property of the two traits in question. Since the
implementations necessarily break the trait guarantees, having the traits
implemented is considered a bug (see [issue 58] for more details). These buggy
implementations were removed. To fix code that used these comparisons without
adding extra allocations, [`SmallRational`][smr-1-22] can be used. For example
`rational == (1, 3)` can be replaced with
`rational == SmallRational::from((1, 3))`.

[ass-1-22]: https://docs.rs/rug/~1.22/rug/trait.Assign.html
[assr-1-22]: https://docs.rs/rug/~1.22/rug/ops/trait.AssignRound.html
[com-1-22]: https://docs.rs/rug/~1.22/rug/struct.Complex.html
[com-e-1-22]: https://docs.rs/rug/~1.22/rug/struct.Complex.html#method.eq0
[com-iz-1-22]: https://docs.rs/rug/~1.22/rug/struct.Complex.html#method.is_zero
[flo-1-22]: https://docs.rs/rug/~1.22/rug/struct.Float.html
[int-1-22]: https://docs.rs/rug/~1.22/rug/struct.Integer.html
[issue 58]: https://gitlab.com/tspiteri/rug/-/issues/58
[rat-1-22]: https://docs.rs/rug/~1.22/rug/struct.Rational.html
[smc-1-22]: https://docs.rs/rug/~1.22/rug/complex/struct.SmallComplex.html
[smf-1-22]: https://docs.rs/rug/~1.22/rug/float/struct.SmallFloat.html
[smi-1-22]: https://docs.rs/rug/~1.22/rug/integer/struct.SmallInteger.html
[smr-1-22]: https://docs.rs/rug/~1.22/rug/rational/struct.SmallRational.html

Version 1.21.0 (2023-08-28)
===========================

  * The following methods were added to [`Integer`][int-1-21]:
      * [`is_zero`][int-iz-1-21], [`is_positive`][int-ip-1-21],
        [`is_negative`][int-in-1-21]
      * [`modulo`][int-m-1-21], [`modulo_mut`][int-mm-1-21],
        [`modulo_from`][int-mf-1-21], [`modulo_ref`][int-mr-1-21]
  * The following methods were added to [`Rational`][rat-1-21]:
      * [`is_zero`][rat-iz-1-21], [`is_positive`][rat-ip-1-21],
        [`is_negative`][rat-in-1-21]
  * Bug fix: panic instead of raising floating-point exception when dividing by
    [`Rational`][rat-1-21] zero ([issue 53]).
  * [`Complete`][c-1-21] was implemented for incomplete values produced by
    [`DivRounding`][dr-1-21] and [`RemRounding`][rr-1-21] implementations
    ([merge request 4]).

[c-1-21]: https://docs.rs/rug/~1.21/rug/trait.Complete.html
[dr-1-21]: https://docs.rs/rug/~1.21/rug/ops/trait.DivRounding.html
[int-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html
[int-in-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.is_negative
[int-ip-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.is_positive
[int-iz-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.is_zero
[int-m-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo
[int-mf-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo_from
[int-mm-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo_mut
[int-mr-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo_ref
[issue 53]: https://gitlab.com/tspiteri/rug/-/issues/53
[merge request 4]: https://gitlab.com/tspiteri/rug/-/merge_requests/4
[rat-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html
[rat-in-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html#method.is_negative
[rat-ip-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html#method.is_positive
[rat-iz-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html#method.is_zero
[rr-1-21]: https://docs.rs/rug/~1.21/rug/ops/trait.RemRounding.html

Version 1.20.1 (2023-08-03)
===========================

  * Bug fix: <code>[Float][flo-1-20]::[to_f32_round][flo-tfr-1-20]</code> was
    rounding some numbers in the subnormal range incorrectly ([issue 56]).

Version 1.20.0 (2023-07-31)
===========================

  * The [gmp-mpfr-sys] dependency was updated to [version 1.6][sys-1-6].
  * The following methods were added to [`Integer`][int-1-20]:
      * [`prev_prime`][int-pp-1-20], [`prev_prime_mut`][int-ppm-1-20],
        [`prev_prime_ref`][int-ppr-1-20]
  * The following associated constants were added to [`Integer`][int-1-20]:
      * [`ONE`][int-o-1-20], [`NEG_ONE`][int-no-1-20]
  * The following associated constants were added to [`Rational`][rat-1-20]:
      * [`ZERO`][rat-z-1-20], [`ONE`][rat-o-1-20], [`NEG_ONE`][rat-no-1-20]

[flo-1-20]: https://docs.rs/rug/~1.20/rug/struct.Float.html
[flo-tfr-1-20]: https://docs.rs/rug/~1.20/rug/struct.Float.html#method.to_f32_round
[int-1-20]: https://docs.rs/rug/~1.20/rug/struct.Integer.html
[int-no-1-20]: https://docs.rs/rug/~1.20/rug/struct.Integer.html#associatedconstant.NEG_ONE
[int-o-1-20]: https://docs.rs/rug/~1.20/rug/struct.Integer.html#associatedconstant.ONE
[int-pp-1-20]: https://docs.rs/rug/~1.20/rug/struct.Integer.html#method.prev_prime
[int-ppm-1-20]: https://docs.rs/rug/~1.20/rug/struct.Integer.html#method.prev_prime_mut
[int-ppr-1-20]: https://docs.rs/rug/~1.20/rug/struct.Integer.html#method.prev_prime_ref
[issue 56]: https://gitlab.com/tspiteri/rug/-/issues/56
[rat-1-20]: https://docs.rs/rug/~1.20/rug/struct.Rational.html
[rat-no-1-20]: https://docs.rs/rug/~1.20/rug/struct.Rational.html#associatedconstant.NEG_ONE
[rat-o-1-20]: https://docs.rs/rug/~1.20/rug/struct.Rational.html#associatedconstant.ONE
[rat-z-1-20]: https://docs.rs/rug/~1.20/rug/struct.Rational.html#associatedconstant.ZERO
[sys-1-6]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html

Version 1.19.2 (2023-03-23)
===========================

  * Bug fix: a zero denominator could be left after catching a panic from
    <code>[Rational][rat-1-19]::[mutate\_numer\_denom][rat-mnd-1-19]</code>
    ([issue 49]).

Version 1.19.1 (2023-02-14)
===========================

  * Bug fix:
    <code>[Rational][rat-1-19]::[mutate\_numer\_denom][rat-mnd-1-19]</code> and
    <code>[Complex][com-1-19]::[mutate\_real\_imag][com-mri-1-19]</code> were
    not unwind-safe ([issue 47]).

Version 1.19.0 (2023-01-06)
===========================

  * The crate now requires rustc version 1.65.0 or later.
  * The [gmp-mpfr-sys] dependency was updated to [version 1.5][sys-1-5].
  * The <code>[Round][r-1-19]::[AwayZero][r-az-1-19]</code> rounding mode was
    added.
  * The following methods were added to [`Float`][flo-1-19]:
      * [`sin_u`][flo-su-1-19], [`sin_u_mut`][flo-sum-1-19],
        [`sin_u_round`][flo-suro-1-19], [`sin_u_ref`][flo-sure-1-19]
      * [`cos_u`][flo-cu-1-19], [`cos_u_mut`][flo-cum-1-19],
        [`cos_u_round`][flo-curo-1-19], [`cos_u_ref`][flo-cure-1-19]
      * [`tan_u`][flo-tu-1-19], [`tan_u_mut`][flo-tum-1-19],
        [`tan_u_round`][flo-turo-1-19], [`tan_u_ref`][flo-ture-1-19]
      * [`sin_pi`][flo-sp-1-19], [`sin_pi_mut`][flo-spm-1-19],
        [`sin_pi_round`][flo-spro-1-19], [`sin_pi_ref`][flo-spre-1-19]
      * [`cos_pi`][flo-cp-1-19], [`cos_pi_mut`][flo-cpm-1-19],
        [`cos_pi_round`][flo-cpro-1-19], [`cos_pi_ref`][flo-cpre-1-19]
      * [`tan_pi`][flo-tp-1-19], [`tan_pi_mut`][flo-tpm-1-19],
        [`tan_pi_round`][flo-tpro-1-19], [`tan_pi_ref`][flo-tpre-1-19]
      * [`asin_u`][flo-asu-1-19], [`asin_u_mut`][flo-asum-1-19],
        [`asin_u_round`][flo-asuro-1-19], [`asin_u_ref`][flo-asure-1-19]
      * [`acos_u`][flo-acu-1-19], [`acos_u_mut`][flo-acum-1-19],
        [`acos_u_round`][flo-acuro-1-19], [`acos_u_ref`][flo-acure-1-19]
      * [`atan_u`][flo-atu-1-19], [`atan_u_mut`][flo-atum-1-19],
        [`atan_u_round`][flo-aturo-1-19], [`atan_u_ref`][flo-ature-1-19]
      * [`atan2_u`][flo-at2u-1-19], [`atan2_u_mut`][flo-at2um-1-19],
        [`atan2_u_round`][flo-at2uro-1-19], [`atan2_u_ref`][flo-at2ure-1-19]
      * [`asin_pi`][flo-asp-1-19], [`asin_pi_mut`][flo-aspm-1-19],
        [`asin_pi_round`][flo-aspro-1-19], [`asin_pi_ref`][flo-aspre-1-19]
      * [`acos_pi`][flo-acp-1-19], [`acos_pi_mut`][flo-acpm-1-19],
        [`acos_pi_round`][flo-acpro-1-19], [`acos_pi_ref`][flo-acpre-1-19]
      * [`atan_pi`][flo-atp-1-19], [`atan_pi_mut`][flo-atpm-1-19],
        [`atan_pi_round`][flo-atpro-1-19], [`atan_pi_ref`][flo-atpre-1-19]
      * [`atan2_pi`][flo-at2p-1-19], [`atan2_pi_mut`][flo-at2pm-1-19],
        [`atan2_pi_round`][flo-at2pro-1-19], [`atan2_pi_ref`][flo-at2pre-1-19]
      * [`log2_1p`][flo-l2-1-19], [`log2_1p_mut`][flo-l2m-1-19],
        [`log2_1p_round`][flo-l2ro-1-19], [`log2_1p_ref`][flo-l2re-1-19]
      * [`log10_1p`][flo-l10-1-19], [`log10_1p_mut`][flo-l10m-1-19],
        [`log10_1p_round`][flo-l10ro-1-19], [`log10_1p_ref`][flo-l10re-1-19]
      * [`exp2_m1`][flo-e2-1-19], [`exp2_m1_mut`][flo-e2m-1-19],
        [`exp2_m1_round`][flo-e2ro-1-19], [`exp2_m1_ref`][flo-e2re-1-19]
      * [`exp10_m1`][flo-e10-1-19], [`exp10_m1_mut`][flo-e10m-1-19],
        [`exp10_m1_round`][flo-e10ro-1-19], [`exp10_m1_ref`][flo-e10re-1-19]
      * [`compound_i`][flo-ci-1-19], [`compound_i_mut`][flo-cim-1-19],
        [`compound_i_round`][flo-ciro-1-19], [`compound_i_ref`][flo-cire-1-19]
      * [`root_i`][flo-ri-1-19], [`root_i_mut`][flo-rim-1-19],
        [`root_i_round`][flo-riro-1-19], [`root_i_ref`][flo-rire-1-19]
  * The following methods were added to [`Complex`][com-1-19]:
      * [`agm`][com-a-1-19], [`agm_mut`][com-am-1-19],
        [`agm_round`][com-aro-1-19], [`agm_ref`][com-are-1-19]
  * The following methods are now const functions:
      * <code>[Integer][int-1-19]::[as\_limbs][int-al-1-19]</code>
      * <code>[Integer][int-1-19]::[as\_neg][int-an-1-19]</code>,
        <code>[Integer][int-1-19]::[as\_abs][int-aa-1-19]</code>
      * <code>[Integer][int-1-19]::[is\_even][int-ie-1-19]</code>,
        <code>[Integer][int-1-19]::[is\_odd][int-io-1-19]</code>
      * <code>[Integer][int-1-19]::[cmp0][int-c-1-19]</code>
      * <code>[Rational][rat-1-19]::[into\_raw][rat-ir-1-19]</code>,
        <code>[Rational][rat-1-19]::[as\_raw][rat-ara-1-19]</code>
      * <code>[Rational][rat-1-19]::[numer][rat-n-1-19]</code>,
        <code>[Rational][rat-1-19]::[denom][rat-d-1-19]</code>
      * <code>[Rational][rat-1-19]::[into\_numer\_denom][rat-ind-1-19]</code>
      * <code>[Rational][rat-1-19]::[as\_neg][rat-an-1-19]</code>,
        <code>[Rational][rat-1-19]::[as\_abs][rat-aa-1-19]</code>,
        <code>[Rational][rat-1-19]::[as\_recip][rat-are-1-19]</code>
      * <code>[Rational][rat-1-19]::[cmp0][rat-c-1-19]</code>
      * <code>[Rational][rat-1-19]::[is\_integer][rat-ii-1-19]</code>
      * <code>[Float][flo-1-19]::[prec][flo-p-1-19]</code>
      * <code>[Float][flo-1-19]::[from\_raw][flo-fr-1-19]</code>,
        <code>[Float][flo-1-19]::[into\_raw][flo-ir-1-19]</code>,
        <code>[Float][flo-1-19]::[as\_raw][flo-ar-1-19]</code>
      * <code>[Float][flo-1-19]::[as\_ord][flo-ao-1-19]</code>,
        <code>[Float][flo-1-19]::[as\_complex][flo-ac-1-19]</code>
      * <code>[Float][flo-1-19]::[is\_nan][flo-ina-1-19]</code>,
        <code>[Float][flo-1-19]::[is\_infinite][flo-ii-1-19]</code>,
        <code>[Float][flo-1-19]::[is\_finite][flo-if-1-19]</code>,
        <code>[Float][flo-1-19]::[is\_zero][flo-iz-1-19]</code>,
        <code>[Float][flo-1-19]::[is\_normal][flo-ino-1-19]</code>,
        <code>[Float][flo-1-19]::[classify][flo-cl-1-19]</code>
      * <code>[Float][flo-1-19]::[cmp0][flo-cm-1-19]</code>
      * <code>[Float][flo-1-19]::[get\_exp][flo-ge-1-19]</code>
      * <code>[Float][flo-1-19]::[is\_sign\_positive][flo-isp-1-19]</code>,
        <code>[Float][flo-1-19]::[is\_sign\_negative][flo-isn-1-19]</code>
      * <code>[Complex][com-1-19]::[prec][com-p-1-19]</code>
      * <code>[Complex][com-1-19]::[from\_raw][com-fr-1-19]</code>,
        <code>[Complex][com-1-19]::[into\_raw][com-ir-1-19]</code>,
        <code>[Complex][com-1-19]::[as\_raw][com-ara-1-19]</code>
      * <code>[Complex][com-1-19]::[real][com-r-1-19]</code>,
        <code>[Complex][com-1-19]::[imag][com-i-1-19]</code>
      * <code>[Complex][com-1-19]::[into\_real\_imag][com-iri-1-19]</code>,
        <code>[Complex][com-1-19]::[borrow\_real\_imag][com-bri-1-19]</code>
      * <code>[Complex][com-1-19]::[as\_ord][com-ao-1-19]</code>
      * <code>[Complex][com-1-19]::[eq0][com-e-1-19]</code>
  * [`BorrowInteger`][bi-1-19], [`BorrowRational`][br-1-19],
    [`BorrowFloat`][bf-1-19] and [`BorrowComplex`][bc-1-19] now implement
    [`Clone`], [`Copy`] and formatting traits; and each have a static method
    `const_deref`.

[bc-1-19]: https://docs.rs/rug/~1.19/rug/complex/struct.BorrowComplex.html
[bf-1-19]: https://docs.rs/rug/~1.19/rug/float/struct.BorrowFloat.html
[bi-1-19]: https://docs.rs/rug/~1.19/rug/integer/struct.BorrowInteger.html
[br-1-19]: https://docs.rs/rug/~1.19/rug/rational/struct.BorrowRational.html
[com-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html
[com-a-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.agm
[com-am-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.agm_mut
[com-ao-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.as_ord
[com-ara-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.as_raw
[com-are-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.agm_ref
[com-aro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.agm_round
[com-bri-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.borrow_real_imag
[com-e-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.eq0
[com-fr-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.from_raw
[com-i-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.imag
[com-ir-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.into_raw
[com-iri-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.into_real_imag
[com-mri-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.mutate_real_imag
[com-p-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.prec
[com-r-1-19]: https://docs.rs/rug/~1.19/rug/struct.Complex.html#method.real
[flo-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html
[flo-ac-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.as_complex
[flo-acp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_pi
[flo-acpm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_pi_mut
[flo-acpre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_pi_ref
[flo-acpro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_pi_round
[flo-acu-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_u
[flo-acum-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_u_mut
[flo-acure-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_u_ref
[flo-acuro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.acos_u_round
[flo-ao-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.as_ord
[flo-ar-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.as_raw
[flo-asp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_pi
[flo-aspm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_pi_mut
[flo-aspre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_pi_ref
[flo-aspro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_pi_round
[flo-asu-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_u
[flo-asum-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_u_mut
[flo-asure-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_u_ref
[flo-asuro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.asin_u_round
[flo-at2p-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_pi
[flo-at2pm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_pi_mut
[flo-at2pre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_pi_ref
[flo-at2pro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_pi_round
[flo-at2u-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_u
[flo-at2um-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_u_mut
[flo-at2ure-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_u_ref
[flo-at2uro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan2_u_round
[flo-atp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_pi
[flo-atpm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_pi_mut
[flo-atpre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_pi_ref
[flo-atpro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_pi_round
[flo-atu-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_u
[flo-atum-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_u_mut
[flo-ature-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_u_ref
[flo-aturo-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.atan_u_round
[flo-ci-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.compound_i
[flo-cim-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.compound_i_mut
[flo-cire-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.compound_i_ref
[flo-ciro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.compound_i_round
[flo-cl-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.classify
[flo-cm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cmp0
[flo-cp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_pi
[flo-cpm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_pi_mut
[flo-cpre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_pi_ref
[flo-cpro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_pi_round
[flo-cu-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_u
[flo-cum-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_u_mut
[flo-cure-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_u_ref
[flo-curo-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.cos_u_round
[flo-e10-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp10_m1
[flo-e10m-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp10_m1_mut
[flo-e10re-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp10_m1_ref
[flo-e10ro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp10_m1_round
[flo-e2-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp2_m1
[flo-e2m-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp2_m1_mut
[flo-e2re-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp2_m1_ref
[flo-e2ro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.exp2_m1_round
[flo-fr-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.from_raw
[flo-ge-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.get_exp
[flo-if-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_finite
[flo-ii-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_infinite
[flo-ina-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_nan
[flo-ino-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_normal
[flo-ir-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.into_raw
[flo-isn-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_sign_negative
[flo-isp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_sign_positive
[flo-iz-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.is_zero
[flo-l10-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log10_1p
[flo-l10m-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log10_1p_mut
[flo-l10re-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log10_1p_ref
[flo-l10ro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log10_1p_round
[flo-l2-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log2_1p
[flo-l2m-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log2_1p_mut
[flo-l2re-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log2_1p_ref
[flo-l2ro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.log2_1p_round
[flo-p-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.prec
[flo-ri-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.root_i
[flo-rim-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.root_i_mut
[flo-rire-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.root_i_ref
[flo-riro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.root_i_round
[flo-sp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_pi
[flo-spm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_pi_mut
[flo-spre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_pi_ref
[flo-spro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_pi_round
[flo-su-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_u
[flo-sum-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_u_mut
[flo-sure-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_u_ref
[flo-suro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.sin_u_round
[flo-tp-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_pi
[flo-tpm-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_pi_mut
[flo-tpre-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_pi_ref
[flo-tpro-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_pi_round
[flo-tu-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_u
[flo-tum-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_u_mut
[flo-ture-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_u_ref
[flo-turo-1-19]: https://docs.rs/rug/~1.19/rug/struct.Float.html#method.tan_u_round
[int-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html
[int-aa-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html#method.as_abs
[int-al-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html#method.as_limbs
[int-an-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html#method.as_neg
[int-c-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html#method.cmp0
[int-ie-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html#method.is_even
[int-io-1-19]: https://docs.rs/rug/~1.19/rug/struct.Integer.html#method.is_odd
[issue 47]: https://gitlab.com/tspiteri/rug/-/issues/47
[issue 49]: https://gitlab.com/tspiteri/rug/-/issues/49
[r-1-19]: https://docs.rs/rug/~1.19/rug/float/enum.Round.html
[r-az-1-19]: https://docs.rs/rug/~1.19/rug/float/enum.Round.html#variant.AwayZero
[rat-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html
[rat-aa-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.as_abs
[rat-an-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.as_neg
[rat-ara-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.as_raw
[rat-are-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.as_recip
[rat-c-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.cmp0
[rat-d-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.denom
[rat-ii-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.is_integer
[rat-ind-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.into_numer_denom
[rat-ir-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.into_raw
[rat-mnd-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.mutate_numer_denom
[rat-n-1-19]: https://docs.rs/rug/~1.19/rug/struct.Rational.html#method.numer
[sys-1-5]: https://docs.rs/gmp-mpfr-sys/~1.5/gmp_mpfr_sys/index.html

Version 1.18.0 (2022-11-17)
===========================

  * For the [`Integer`][int-1-18] struct, the methods
    [`gcd_cofactors`][int-gc-1-18], [`gcd_cofactors_mut`][int-gcm-1-18] and
    [`gcd_cofactors_ref`][int-gcr-1-18] were renamed to
    [`extended_gcd`][int-eg-1-18], [`extended_gcd_mut`][int-egm-1-18] and
    [`extended_gcd_ref`][int-egr-1-18]. The old method names are deprecated.
  * All error types now implement [`Clone`], [`Copy`], [`PartialEq`] and [`Eq`].
  * The [`IntegerExt64`][ie64-1-18] extension trait was added to support very
    large integers better on some 64-bit platforms.

[ie64-1-18]: https://docs.rs/rug/~1.18/rug/integer/trait.IntegerExt64.html
[int-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html
[int-eg-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html#method.extended_gcd
[int-egm-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html#method.extended_gcd_mut
[int-egr-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html#method.extended_gcd_ref
[int-gc-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html#method.gcd_cofactors
[int-gcm-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html#method.gcd_cofactors_mut
[int-gcr-1-18]: https://docs.rs/rug/~1.18/rug/struct.Integer.html#method.gcd_cofactors_ref

Version 1.17.0 (2022-08-06)
===========================

  * The new method
    <code>[Integer][int-1-17]::[assign\_bytes\_radix\_unchecked][int-abru-1-17]</code>
    was added ([issue 41]).
  * [`Integer`][int-1-17] now implements [`LowerExp`] and [`UpperExp`] for
    convenience by transparently converting the [`Integer`][int-1-17] to a
    [`Float`][flo-1-17] ([issue 42]).
  * Bug fix: the [`CompleteRound`][cr-1-17] trait is now implemented for
      * the value returned from
        <code>[Float][flo-1-17]::[mul\_add\_mul\_ref][flo-mamr-1-17]</code>
      * the value returned from
        <code>[Float][flo-1-17]::[mul\_sub\_mul\_ref][flo-msmr-1-17]</code>
  * The [`CompleteRound`][cr-1-17] trait is now also implemented for
      * the value returned from negating a reference to [`Float`][flo-1-17],
        that is
        <code>&lt;&amp;[Float][flo-1-17] as [Neg][`Neg`]>::[Output][NegOutput]</code>
      * the value returned from negating a reference to [`Complex`][com-1-17],
        that is
        <code>&lt;&amp;[Complex][com-1-17] as [Neg][`Neg`]>::[Output][NegOutput]</code>

[com-1-17]: https://docs.rs/rug/~1.17/rug/struct.Complex.html
[cr-1-17]: https://docs.rs/rug/~1.17/rug/ops/trait.CompleteRound.html
[flo-1-17]: https://docs.rs/rug/~1.17/rug/struct.Float.html
[flo-mamr-1-17]: https://docs.rs/rug/~1.17/rug/struct.Float.html#method.mul_add_mul_ref
[flo-msmr-1-17]: https://docs.rs/rug/~1.17/rug/struct.Float.html#method.mul_sub_mul_ref
[int-1-17]: https://docs.rs/rug/~1.17/rug/struct.Integer.html
[int-abru-1-17]: https://docs.rs/rug/~1.17/rug/struct.Integer.html#method.assign_bytes_radix_unchecked
[issue 41]: https://gitlab.com/tspiteri/rug/-/issues/41
[issue 42]: https://gitlab.com/tspiteri/rug/-/issues/42

Version 1.16.0 (2022-04-28)
===========================

  * Now [`Float`][flo-1-16] implements [`Cast`][az-c-1-1],
    [`CheckedCast`][az-cc-1-1] and [`UnwrappedCast`][az-uc-1-1] with primitive
    integers as the destination types, in addition to the previously supported
    [`SaturatingCast`][az-sc-1-1].
  * The new methods <code>[Float][flo-1-16]::[total\_cmp][flo-tc-1-16]</code>
    and <code>[Complex][com-1-16]::[total\_cmp][com-tc-1-16]</code> were added.
  * The new methods
    <code>[Complex][com-1-16]::[borrow\_real\_imag][com-bri-1-16]</code> and
    <code>[Complex][com-1-16]::[mutate\_real\_imag][com-mri-1-16]</code> were
    added.
  * Arithmetic operators are overloaded to work on combinations of Rug types and
    [`isize`] and [`usize`].
  * The [`BorrowInteger`][bi-1-16], [`BorrowRational`][br-1-16],
    [`BorrowFloat`][bf-1-16] and [`BorrowComplex`][bc-1-16] structs were added.

[az-c-1-1]: https://docs.rs/az/1.1/az/trait.Cast.html
[az-cc-1-1]: https://docs.rs/az/1.1/az/trait.CheckedCast.html
[az-sc-1-1]: https://docs.rs/az/1.1/az/trait.SaturatingCast.html
[az-uc-1-1]: https://docs.rs/az/1.1/az/trait.UnwrappedCast.html
[bc-1-16]: https://docs.rs/rug/~1.16/rug/complex/struct.BorrowComplex.html
[bf-1-16]: https://docs.rs/rug/~1.16/rug/float/struct.BorrowFloat.html
[bi-1-16]: https://docs.rs/rug/~1.16/rug/integer/struct.BorrowInteger.html
[br-1-16]: https://docs.rs/rug/~1.16/rug/rational/struct.BorrowRational.html
[com-1-16]: https://docs.rs/rug/~1.16/rug/struct.Complex.html
[com-bri-1-16]: https://docs.rs/rug/~1.16/rug/struct.Complex.html#method.borrow_real_imag
[com-mri-1-16]: https://docs.rs/rug/~1.16/rug/struct.Complex.html#method.mutate_real_imag
[com-tc-1-16]: https://docs.rs/rug/~1.16/rug/struct.Complex.html#method.total_cmp
[flo-1-16]: https://docs.rs/rug/~1.16/rug/struct.Float.html
[flo-tc-1-16]: https://docs.rs/rug/~1.16/rug/struct.Float.html#method.total_cmp

Version 1.15.0 (2022-02-03)
===========================

  * Now [`Integer`][int-1-15], [`Rational`][rat-1-15], [`Float`][flo-1-15] and
    [`Complex`][com-1-15] implement [`Shl`], [`Shr`], [`ShlAssign`] and
    [`ShrAssign`] with [`isize`] and [`usize`] as the type of the right-hand
    side, in addition to the previously supported [`i32`] and [`u32`].
  * The new experimental feature [`num-traits`][feat-nt-1-15] was added to
    implement some traits from the [*num-traits* crate] and the [*num-integer*
    crate] ([issue 30]).

[com-1-15]: https://docs.rs/rug/~1.15/rug/struct.Complex.html
[feat-nt-1-15]: https://docs.rs/rug/~1.15/rug/index.html#experimental-optional-features
[flo-1-15]: https://docs.rs/rug/~1.15/rug/struct.Float.html
[int-1-15]: https://docs.rs/rug/~1.15/rug/struct.Integer.html
[issue 30]: https://gitlab.com/tspiteri/rug/-/issues/30
[rat-1-15]: https://docs.rs/rug/~1.15/rug/struct.Rational.html

Version 1.14.1 (2022-01-23)
===========================

  * Bug fix: <code>[Rational][rat-1-14]::[to_f32][rat-tf-1-14]</code> was
    rounding away from zero in some cases when the rational number should be
    converted to a subnormal number ([issue 36]).

Version 1.14.0 (2021-11-24)
===========================

  * Bug fix: [`OrdFloat`][ofl-1-14] was incorrectly ordering +NaN < &minus;∞.
  * The <code>[Integer][int-1-14]::[ZERO][int-z-1-14]</code> associated constant
    was added.
  * The <code>[Integer][int-1-14]::[shrink_to][int-st-1-14]</code> method was
    added.
  * The <code>[Rational][rat-1-14]::[is_integer][rat-ii-1-14]</code> method was
    added.
  * The <code>[Complete][comp-1-14]::[complete_into][comp-ci-1-14]</code>
    provided method was added to the [`Complete`][comp-1-14] trait.
  * The [`CompleteRound`][compr-1-14] trait was added to make it easier to
    convert [`Float`][flo-1-14] and [`Complex`][com-1-14] numbers
    [incomplete-computation values][icv-1-14] to their final value.

[com-1-14]: https://docs.rs/rug/~1.14/rug/struct.Complex.html
[comp-1-14]: https://docs.rs/rug/~1.14/rug/trait.Complete.html
[comp-ci-1-14]: https://docs.rs/rug/~1.14/rug/trait.Complete.html#method.complete_into
[compr-1-14]: https://docs.rs/rug/~1.14/rug/ops/trait.CompleteRound.html
[flo-1-14]: https://docs.rs/rug/~1.14/rug/struct.Float.html
[icv-1-14]: https://docs.rs/rug/~1.14/rug/index.html#incomplete-computation-values
[int-1-14]: https://docs.rs/rug/~1.14/rug/struct.Integer.html
[int-st-1-14]: https://docs.rs/rug/~1.14/rug/struct.Integer.html#method.shrink_to
[int-z-1-14]: https://docs.rs/rug/~1.14/rug/struct.Integer.html#associatedconstant.ZERO
[issue 36]: https://gitlab.com/tspiteri/rug/-/issues/36
[ofl-1-14]: https://docs.rs/rug/~1.14/rug/float/struct.OrdFloat.html
[rat-1-14]: https://docs.rs/rug/~1.14/rug/struct.Rational.html
[rat-ii-1-14]: https://docs.rs/rug/~1.14/rug/struct.Rational.html#method.is_integer
[rat-tf-1-14]: https://docs.rs/rug/~1.14/rug/struct.Rational.html#method.to_f32

Version 1.13.0 (2021-07-29)
===========================

  * The [incomplete-computation value][icv-1-13] returned by
    <code>[Integer][int-1-13]::{[sum][int-s-1-13],[dot][int-d-1-13]}</code> can
    now be subtracted from an [`Integer`][int-1-13].
  * The [incomplete-computation value][icv-1-13] returned by
    <code>[Rational][rat-1-13]::{[sum][rat-s-1-13],[dot][rat-d-1-13]}</code> can
    now be subtracted from a [`Rational`][rat-1-13] number.
  * The [incomplete-computation value][icv-1-13] returned by
    <code>[Float][flo-1-13]::{[sum][flo-s-1-13],[dot][flo-d-1-13]}</code> can
    now be subtracted from a [`Float`][flo-1-13].
  * The [incomplete-computation value][icv-1-13] returned by
    <code>[Complex][com-1-13]::{[sum][com-s-1-13],[dot][com-d-1-13]}</code> can
    now be subtracted from a [`Complex`][com-1-13] number.
  * The [incomplete-computation value][icv-1-13] returned by
    <code>[Integer][int-1-13]::[square\_ref][int-sr-1-13]</code> can now be
    added to/subtracted from an [`Integer`][int-1-13].
  * The [`Complete`][comp-1-13] trait was added to make it easier to convert
    [incomplete-computation values][icv-1-13] to their final value.
  * The <code>[Round][rnd-1-13]::[reverse][rnd-r-1-13]</code> method was added.

[com-1-13]: https://docs.rs/rug/~1.13/rug/struct.Complex.html
[com-d-1-13]: https://docs.rs/rug/~1.13/rug/struct.Complex.html#method.dot
[com-s-1-13]: https://docs.rs/rug/~1.13/rug/struct.Complex.html#method.sum
[comp-1-13]: https://docs.rs/rug/~1.13/rug/trait.Complete.html
[flo-1-13]: https://docs.rs/rug/~1.13/rug/struct.Float.html
[flo-d-1-13]: https://docs.rs/rug/~1.13/rug/struct.Float.html#method.dot
[flo-s-1-13]: https://docs.rs/rug/~1.13/rug/struct.Float.html#method.sum
[icv-1-13]: https://docs.rs/rug/~1.13/rug/index.html#incomplete-computation-values
[int-1-13]: https://docs.rs/rug/~1.13/rug/struct.Integer.html
[int-d-1-13]: https://docs.rs/rug/~1.13/rug/struct.Integer.html#method.dot
[int-s-1-13]: https://docs.rs/rug/~1.13/rug/struct.Integer.html#method.sum
[int-sr-1-13]: https://docs.rs/rug/~1.13/rug/struct.Integer.html#method.square_ref
[rat-1-13]: https://docs.rs/rug/~1.13/rug/struct.Rational.html
[rat-d-1-13]: https://docs.rs/rug/~1.13/rug/struct.Rational.html#method.dot
[rat-s-1-13]: https://docs.rs/rug/~1.13/rug/struct.Rational.html#method.sum
[rnd-1-13]: https://docs.rs/rug/~1.13/rug/float/enum.Round.html
[rnd-r-1-13]: https://docs.rs/rug/~1.13/rug/float/enum.Round.html#method.reverse

Version 1.12.0 (2021-03-25)
===========================

  * A new method <code>[Integer][int-1-12]::[as\_limbs][int-al-1-12]</code> was
    added.
  * The [*az* crate] dependency was updated to [version 1.1][az-1-1].

[az-1-1]: https://docs.rs/az/~1.1/az/index.html
[int-1-12]: https://docs.rs/rug/~1.12/rug/struct.Integer.html
[int-al-1-12]: https://docs.rs/rug/~1.12/rug/struct.Integer.html#method.as_limbs

Version 1.11.0 (2020-09-03)
===========================

  * The [gmp-mpfr-sys] dependency was updated to [version 1.4][sys-1-4].
  * Now it is possible to display [`Float`][flo-1-11] numbers with only one
    siginificant digit.

[flo-1-11]: https://docs.rs/rug/~1.11/rug/struct.Float.html
[sys-1-4]: https://docs.rs/gmp-mpfr-sys/~1.4/gmp_mpfr_sys/index.html

Version 1.10.0 (2020-07-23)
===========================

  * The [gmp-mpfr-sys] dependency was updated to [version 1.3][sys-1-3].
  * Now <code>[Option][`Option`]&lt;[Integer][int-1-10]&gt;</code> has the same
    size as [`Integer`][int-1-10]; and similar for [`Rational`][rat-1-10],
    [`Float`][flo-1-10], [`Complex`][com-1-10], [`RandState`][ras-1-10] and
    [`ThreadRandState`][trs-1-10].

[com-1-10]: https://docs.rs/rug/~1.10/rug/struct.Complex.html
[flo-1-10]: https://docs.rs/rug/~1.10/rug/struct.Float.html
[int-1-10]: https://docs.rs/rug/~1.10/rug/struct.Integer.html
[ras-1-10]: https://docs.rs/rug/~1.10/rug/rand/struct.RandState.html
[rat-1-10]: https://docs.rs/rug/~1.10/rug/struct.Rational.html
[sys-1-3]: https://docs.rs/gmp-mpfr-sys/~1.3/gmp_mpfr_sys/index.html
[trs-1-10]: https://docs.rs/rug/~1.10/rug/rand/struct.ThreadRandState.html

Version 1.9.0 (2020-06-01)
==========================

  * Displaying [`Float`][flo-1-9] and [`Complex`][com-1-9] numbers, and
    converting them to strings, now avoids outputting an exponent term if the
    radix point can be moved to the correct place without inserting any extra
    digits. For example `"42.0"` will be produced rather than `"4.20e1"` ([issue
    18]). This change does not affect output when [`LowerExp`] (`"{:e}"` format)
    or [`UpperExp`] (`"{:E}"` format) is used.
  * New methods
    <code>[Float][flo-1-9]::[to\_sign\_string\_exp][flo-tsse-1-9]</code> and
    <code>[Float][flo-1-9]::[to\_sign\_string\_exp\_round][flo-tsser-1-9]</code>
    were added.
  * A new function
    <code>[float][flom-1-9]::[allowed\_exp\_range][flom-aer-1-9]</code> was
    added.
  * A new method <code>[Float][flo-1-9]::[clamp\_exp][flo-ce-1-9]</code> was
    added.
  * The following methods are now const functions:
      * <code>[Integer][int-1-9]::[from\_raw][int-fr-1-9]</code>,
        <code>[Rational][rat-1-9]::[from\_raw][rat-fr-1-9]</code>
  * The [*az* crate] is now a public dependency, and wrapping and checked casts
    to/from primitives and big numbers are provided through the traits of the
    crate.

Compatibility note
------------------

The output of [`Float`][flo-1-9] and [`Complex`][com-1-9] numbers was changed as
specified above.

[com-1-9]: https://docs.rs/rug/~1.9/rug/struct.Complex.html
[flo-1-9]: https://docs.rs/rug/~1.9/rug/struct.Float.html
[flo-ce-1-9]: https://docs.rs/rug/~1.9/rug/struct.Float.html#method.clamp_exp
[flo-tsse-1-9]: https://docs.rs/rug/~1.9/rug/struct.Float.html#method.to_sign_string_exp
[flo-tsser-1-9]: https://docs.rs/rug/~1.9/rug/struct.Float.html#method.to_sign_string_exp_round
[flom-1-9]: https://docs.rs/rug/~1.9/rug/float/index.html
[flom-aer-1-9]: https://docs.rs/rug/~1.9/rug/float/fn.allowed_exp_range.html
[int-1-9]: https://docs.rs/rug/~1.9/rug/struct.Integer.html
[int-fr-1-9]: https://docs.rs/rug/~1.9/rug/struct.Integer.html#method.from_raw
[issue 18]: https://gitlab.com/tspiteri/rug/-/issues/18
[rat-1-9]: https://docs.rs/rug/~1.9/rug/struct.Rational.html
[rat-fr-1-9]: https://docs.rs/rug/~1.9/rug/struct.Rational.html#method.from_raw

Version 1.8.0 (2020-04-08)
==========================

  * The following methods are now const functions:
      * <code>[Integer][int-1-8]::[new][int-n-1-8]</code>,
	    <code>[Integer][int-1-8]::[into\_raw][int-ir-1-8]</code>,
	    <code>[Integer][int-1-8]::[as\_raw][int-araw-1-8]</code>
      * <code>[SmallInteger][smi-1-8]::[new][smi-n-1-8]</code>,
        <code>[SmallRational][smr-1-8]::[new][smr-n-1-8]</code>
  * A new method <code>[Integer][int-1-8]::[as\_rational][int-arat-1-8]</code>
    was added.
  * A new method <code>[Float][flo-1-8]::[as\_complex][flo-ac-1-8]</code> was
    added.
  * [`SmallFloat`][smf-1-8] and [`SmallComplex`][smc-1-8] can now be initialized
    with [`Special`][spe-1-8].
  * New methods <code>[SmallFloat][smf-1-8]::[new][smf-n-1-8]</code> and
    <code>[SmallComplex][smc-1-8]::[new][smc-n-1-8]</code> were added.
  * [`SmallFloat`][smf-1-8] and [`SmallComplex`][smc-1-8] now implement
    [`Default`].
  * [`Integer`][int-1-8] now implements
    <code>[AsRef][`AsRef`]&lt;[\[][slice][limb_t][sys-limb-1-2][\]][slice]&gt;</code>.
  * [`Float`][flo-1-8] now implements
    <code>[AsRef][`AsRef`]&lt;[OrdFloat][of-1-8]&gt;</code>, and
    [`OrdFloat`][of-1-8] now implements
    <code>[AsRef][`AsRef`]&lt;[Float][flo-1-8]&gt;</code> and
    <code>[AsMut][`AsMut`]&lt;[Float][flo-1-8]&gt;</code>.
  * [`Complex`][com-1-8] now implements
    <code>[AsRef][`AsRef`]&lt;[OrdComplex][oc-1-8]&gt;</code>, and
    [`OrdComplex`][oc-1-8] now implements
    <code>[AsRef][`AsRef`]&lt;[Complex][com-1-8]&gt;</code> and
    <code>[AsMut][`AsMut`]&lt;[Complex][com-1-8]&gt;</code>.

[com-1-8]: https://docs.rs/rug/~1.8/rug/struct.Complex.html
[flo-1-8]: https://docs.rs/rug/~1.8/rug/struct.Float.html
[flo-ac-1-8]: https://docs.rs/rug/~1.8/rug/struct.Float.html#method.as_complex
[int-1-8]: https://docs.rs/rug/~1.8/rug/struct.Integer.html
[int-arat-1-8]: https://docs.rs/rug/~1.8/rug/struct.Integer.html#method.as_rational
[int-araw-1-8]: https://docs.rs/rug/~1.8/rug/struct.Integer.html#method.as_raw
[int-ir-1-8]: https://docs.rs/rug/~1.8/rug/struct.Integer.html#method.into_raw
[int-n-1-8]: https://docs.rs/rug/~1.8/rug/struct.Integer.html#method.new
[oc-1-8]: https://docs.rs/rug/~1.8/rug/complex/struct.OrdComplex.html
[of-1-8]: https://docs.rs/rug/~1.8/rug/float/struct.OrdFloat.html
[smc-1-8]: https://docs.rs/rug/~1.8/rug/complex/struct.SmallComplex.html
[smc-n-1-8]: https://docs.rs/rug/~1.8/rug/complex/struct.SmallComplex.html#method.new
[smf-1-8]: https://docs.rs/rug/~1.8/rug/float/struct.SmallFloat.html
[smf-n-1-8]: https://docs.rs/rug/~1.8/rug/float/struct.SmallFloat.html#method.new
[smi-1-8]: https://docs.rs/rug/~1.8/rug/integer/struct.SmallInteger.html
[smi-n-1-8]: https://docs.rs/rug/~1.8/rug/integer/struct.SmallInteger.html#method.new
[smr-1-8]: https://docs.rs/rug/~1.8/rug/rational/struct.SmallRational.html
[smr-n-1-8]: https://docs.rs/rug/~1.8/rug/rational/struct.SmallRational.html#method.new
[spe-1-8]: https://docs.rs/rug/~1.8/rug/float/enum.Special.html
[sys-limb-1-2]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/gmp/type.limb_t.html

Version 1.7.0 (2020-01-21)
==========================

  * The crate now requires rustc version 1.37.0 or later.
  * The [gmp-mpfr-sys] dependency was updated to [version 1.2][sys-1-2].
  * The [`RemAssignRound`][rar-1-7] and [`RemFromRound`][rfr-1-7] traits were
    added.
  * The [`Rem`], [`RemAssign`], [`RemFrom`][rf-1-7], [`RemAssignRound`][rar-1-7]
    and [`RemFromRound`][rfr-1-7] traits were implemented for
    [`Float`][flo-1-7].
  * Arithmetic operations with one [`Integer`][int-1-7] or integer primitive
    operand and one [`Rational`][rat-1-7] operand were added.
  * A new method
    <code>[Integer][int-1-7]::[div\_exact\_from][int-def-1-7]</code> was added.
  * New methods <code>[Integer][int-1-7]::[gcd\_u][int-gu-1-7]</code>,
	<code>[Integer][int-1-7]::[gcd\_u\_mut][int-gum-1-7]</code> and
	<code>[Integer][int-1-7]::[gcd\_u\_ref][int-gur-1-7]</code>were added.
  * New methods <code>[Integer][int-1-7]::[lcm\_u][int-lu-1-7]</code>,
	<code>[Integer][int-1-7]::[lcm\_u\_mut][int-lum-1-7]</code> and
	<code>[Integer][int-1-7]::[lcm\_u\_ref][int-lur-1-7]</code>were added.
  * New methods <code>[Float][flo-1-7]::[remainder][flo-r-1-7]</code>,
    <code>[Float][flo-1-7]::[remainder\_mut][flo-rm-1-7]</code>,
    <code>[Float][flo-1-7]::[remainder\_round][flo-rr-1-7]</code>,
    <code>[Float][flo-1-7]::[remainder\_from][flo-rf-1-7]</code>,
    <code>[Float][flo-1-7]::[remainder\_from\_round][flo-rfr-1-7]</code> and
    <code>[Float][flo-1-7]::[remainder\_ref][flo-rre-1-7]</code> were added.

Compatibility note
------------------

  * [`SmallInteger`][smi-1-7], [`SmallRational`][smr-1-7],
    [`SmallFloat`][smf-1-7] and [`SmallComplex`][smc-1-7] are no longer [`Sync`]
    to avoid the possibility of a [data race][dr-1-7]. References obtained by
    dereferencing them, for example the <code>&amp;[Integer][int-1-7]</code>
    returned from <code>[SmallInteger][smi-1-7]::[deref][`deref`]</code>, are
    still [`Send`] and [`Sync`].

[dr-1-7]: https://internals.rust-lang.org/t/is-this-a-data-race/11582
[flo-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html
[flo-r-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html#method.remainder
[flo-rf-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html#method.remainder_from
[flo-rfr-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html#method.remainder_from_round
[flo-rm-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html#method.remainder_mut
[flo-rr-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html#method.remainder_round
[flo-rre-1-7]: https://docs.rs/rug/~1.7/rug/struct.Float.html#method.remainder_ref
[int-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html
[int-def-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.div_exact_from
[int-gu-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.gcd_u
[int-gum-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.gcd_u_mut
[int-gur-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.gcd_u_ref
[int-lu-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.lcm_u
[int-lum-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.lcm_u_mut
[int-lur-1-7]: https://docs.rs/rug/~1.7/rug/struct.Integer.html#method.lcm_u_ref
[rar-1-7]: https://docs.rs/rug/~1.7/rug/ops/trait.RemAssignRound.html
[rat-1-7]: https://docs.rs/rug/~1.7/rug/struct.Rational.html
[rf-1-7]: https://docs.rs/rug/~1.7/rug/ops/trait.RemFrom.html
[rfr-1-7]: https://docs.rs/rug/~1.7/rug/ops/trait.RemFromRound.html
[smc-1-7]: https://docs.rs/rug/~1.7/rug/complex/struct.SmallComplex.html
[smf-1-7]: https://docs.rs/rug/~1.7/rug/float/struct.SmallFloat.html
[smi-1-7]: https://docs.rs/rug/~1.7/rug/integer/struct.SmallInteger.html
[smr-1-7]: https://docs.rs/rug/~1.7/rug/rational/struct.SmallRational.html
[sys-1-2]: https://docs.rs/gmp-mpfr-sys/~1.2/gmp_mpfr_sys/index.html

Version 1.6.0 (2019-09-03)
==========================

  * Arithmetic operator implementations for [`i8`], [`i16`], [`i64`],
    [`i128`], [`u8`], [`u16`], [`u64`] and [`u128`] were added to the
    existing implementations for [`i32`] and [`u32`].
  * A new function
    <code>[float][flom-1-6]::[free\_cache][fc-1-6]</code> and its
    supporting enum [`FreeCache`][fce-1-6] were added.
  * A new method
    <code>[RandState][ras-1-6]::[into\_custom\_boxed][ras-icb-1-6]</code>
    was added.
  * A new struct [`ThreadRandState`][trs-1-6] and a new trait
    [`ThreadRandGen`][trg-1-6] were added to the [`rand`][ram-1-6]
    module to enable the use of custom random generators that are not
    [`Send`] and [`Sync`].
  * Numeric type methods which take [`RandState`][ras-1-6] now can
    also take [`ThreadRandState`][trs-1-6].

Compatibility note
------------------

  * The numeric type methods which took <code>&mut [RandState][ras-1-6]</code>
    were changed to take <code>&mut dyn [MutRandState][mrs-1-6]</code> instead.
    Under normal use, this does not have any affect apart from allowing the use
    of <code>&mut [ThreadRandState][trs-1-6]</code> as well. With function
    inlining and optimization, the generated code should have the same
    performance.

[fc-1-6]: https://docs.rs/rug/~1.6/rug/float/fn.free_cache.html
[fce-1-6]: https://docs.rs/rug/~1.6/rug/float/enum.FreeCache.html
[flom-1-6]: https://docs.rs/rug/~1.6/rug/float/index.html
[mrs-1-6]: https://docs.rs/rug/~1.6/rug/rand/trait.MutRandState.html
[ram-1-6]: https://docs.rs/rug/~1.6/rug/rand/index.html
[ras-1-6]: https://docs.rs/rug/~1.6/rug/rand/struct.RandState.html
[ras-icb-1-6]: https://docs.rs/rug/~1.6/rug/rand/struct.RandState.html#method.into_custom_boxed
[trg-1-6]: https://docs.rs/rug/~1.6/rug/rand/trait.ThreadRandGen.html
[trs-1-6]: https://docs.rs/rug/~1.6/rug/rand/struct.ThreadRandState.html

Version 1.5.2 (2019-07-26)
==========================

  * Bug fix: <code>[Pow][pow-1-5]&lt;[i32][`i32`]&gt; for
    [Rational][rat-1-5]</code> was returning the reciprocal of the correct
    result.

Version 1.5.1 (2019-07-10)
==========================

  * Bug fix: a memory leak in conversions of [`Float`][flo-1-5] to string was
    fixed (https://gitlab.com/tspiteri/rug/-/issues/11).

Version 1.5.0 (2019-07-04)
==========================

  * A new method
    <code>[Integer][int-1-5]::[assign\_digits\_unaligned][int-adu-1-5]</code>
    was added to enable reading digits from unaligned memory.
  * A new method
    <code>[Integer][int-1-5]::[write\_digits\_unaligned][int-wdu-1-5]</code> was
    added to enable writing digits to unaligned and uninitialized memory.
  * New methods <code>[Float][flo-1-5]::[u\_exp][flo-ue-1-5]</code> and
    <code>[Float][flo-1-5]::[i\_exp][flo-ie-1-5]</code> were added.
  * A new method <code>[Complex][com-1-5]::[abs\_round][com-ar-1-5]</code> was
    added.
  * The documentation examples on [`from_raw`][fr-1-5] methods now use
    [`MaybeUninit`] instead of
    <code>[mem][`mem`]::[uninitialized][`uninitialized`]</code>.

[com-1-5]: https://docs.rs/rug/~1.5/rug/struct.Complex.html
[com-ar-1-5]: https://docs.rs/rug/~1.5/rug/struct.Complex.html#method.abs_round
[flo-1-5]: https://docs.rs/rug/~1.5/rug/struct.Float.html
[flo-ie-1-5]: https://docs.rs/rug/~1.5/rug/struct.Float.html#method.i_exp
[flo-ue-1-5]: https://docs.rs/rug/~1.5/rug/struct.Float.html#method.u_exp
[fr-1-5]: https://docs.rs/rug/~1.5/rug/struct.Integer.html#method.from_raw
[int-1-5]: https://docs.rs/rug/~1.5/rug/struct.Integer.html
[int-adu-1-5]: https://docs.rs/rug/~1.5/rug/struct.Integer.html#method.assign_digits_unaligned
[int-wdu-1-5]: https://docs.rs/rug/~1.5/rug/struct.Integer.html#method.write_digits_unaligned
[pow-1-5]: https://docs.rs/rug/~1.5/rug/ops/trait.Pow.html
[rat-1-5]: https://docs.rs/rug/~1.5/rug/struct.Rational.html

Version 1.4.0 (2019-04-24)
==========================

  * The crate now requires rustc version 1.31.0 or later.
  * The method <code>[RandState][ras-1-4]::[as\_raw][ras-ar-1-4]</code> was
    fixed to take `&self` instead of `&mut self`.
  * <code>[float][flom-1-0]::[prec\_min][flom-pmin-1-4]</code> and
    <code>[float][flom-1-0]::[prec\_max][flom-pmax-1-4]</code> are now const
    functions.
  * New methods <code>[Float][flo-1-4]::[copysign][flo-cs-1-4]</code>,
    <code>[Float][flo-1-4]::[copysign\_mut][flo-csm-1-4]</code> and
    <code>[Float][flo-1-4]::[copysign\_ref][flo-csr-1-4]</code> were added.

[flo-1-4]: https://docs.rs/rug/~1.4/rug/struct.Float.html
[flo-cs-1-4]: https://docs.rs/rug/~1.4/rug/struct.Float.html#method.copysign
[flo-csm-1-4]: https://docs.rs/rug/~1.4/rug/struct.Float.html#method.copysign_mut
[flo-csr-1-4]: https://docs.rs/rug/~1.4/rug/struct.Float.html#method.copysign_ref
[flom-1-4]: https://docs.rs/rug/~1.4/rug/float/index.html
[flom-pmax-1-4]: https://docs.rs/rug/~1.4/rug/float/fn.prec_max.html
[flom-pmin-1-4]: https://docs.rs/rug/~1.4/rug/float/fn.prec_min.html
[ras-1-4]: https://docs.rs/rug/~1.4/rug/rand/struct.RandState.html
[ras-ar-1-4]: https://docs.rs/rug/~1.4/rug/rand/struct.RandState.html#method.as_raw

Version 1.3.0 (2019-01-26)
==========================

  * A new method
    <code>[SmallRational][smr-1-3]::[assign\_canonical][smr-ac-1-3]</code> was
    added.

[smr-1-3]: https://docs.rs/rug/~1.3/rug/rational/struct.SmallRational.html
[smr-ac-1-3]: https://docs.rs/rug/~1.3/rug/rational/struct.SmallRational.html#method.assign_canonical

Version 1.2.3 (2019-01-21)
==========================

  * Fixed bug in <code>[Integer][int-1-2]::[assign\_digits][int-ad-1-2]</code>.
    (Thanks: Eric Scrivner)

Version 1.2.2 (2018-10-18)
==========================

  * The [`NotAssign`][noa-1-2], [`BitAndFrom`][baf-1-2], [`BitOrFrom`][bof-1-2]
    and [`BitXorFrom`][bxf-1-2] traits were implemented for [`bool`].
  * The [`NegAssign`][nea-1-2] trait was implemented for [`f32`] and [`f64`].

Version 1.2.1 (2018-08-16)
==========================

  * The [`Integer`][int-1-2] methods [`from_digits`][int-fd-1-2],
    [`assign_digits`][int-ad-1-2], [`significant_digits`][int-sd-1-2],
    [`to_digits`][int-td-1-2] and [`write_digits`][int-wd-1-2] now support
    [`bool`] [slices][slice].

Version 1.2.0 (2018-06-30)
==========================

  * The implementations of [`Sum`] and [`Product`] for [`Integer`][int-1-2] and
    [`Rational`][rat-1-2] were generalized to accept more types of elements.
  * New methods
    <code>[Integer][int-1-2]::[keep\_signed\_bits][int-ksb-1-2]</code>,
    <code>[Integer][int-1-2]::[keep\_signed\_bits\_mut][int-ksbm-1-2]</code> and
    <code>[Integer][int-1-2]::[keep\_signed\_bits\_ref][int-ksbr-1-2]</code>
    were added.
  * New methods <code>[Integer][int-1-2]::[sum][int-s-1-2]</code>,
    <code>[Integer][int-1-2]::[dot][int-d-1-2]</code> and
    <code>[Integer][int-1-2]::[product][int-p-1-2]</code> were added
  * New methods <code>[Rational][rat-1-2]::[sum][rat-s-1-2]</code>,
    <code>[Rational][rat-1-2]::[dot][rat-d-1-2]</code> and
    <code>[Rational][rat-1-2]::[product][rat-p-1-2]</code> were added.
  * New methods <code>[Float][flo-1-2]::[dot][flo-d-1-2]</code> and
    <code>[Complex][com-1-2]::[dot][com-d-1-2]</code> were added.

[baf-1-2]: https://docs.rs/rug/~1.2/rug/ops/trait.BitAndFrom.html
[bof-1-2]: https://docs.rs/rug/~1.2/rug/ops/trait.BitOrFrom.html
[bxf-1-2]: https://docs.rs/rug/~1.2/rug/ops/trait.BitXorFrom.html
[com-1-2]: https://docs.rs/rug/~1.2/rug/struct.Complex.html
[com-d-1-2]: https://docs.rs/rug/~1.2/rug/struct.Complex.html#method.dot
[flo-1-2]: https://docs.rs/rug/~1.2/rug/struct.Float.html
[flo-d-1-2]: https://docs.rs/rug/~1.2/rug/struct.Float.html#method.dot
[int-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html
[int-ad-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.assign_digits
[int-d-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.dot
[int-fd-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.from_digits
[int-ksb-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.keep_signed_bits
[int-ksbm-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.keep_signed_bits_mut
[int-ksbr-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.keep_signed_bits_ref
[int-p-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.product
[int-s-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.sum
[int-sd-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.significant_digits
[int-td-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.to_digits
[int-wd-1-2]: https://docs.rs/rug/~1.2/rug/struct.Integer.html#method.write_digits
[nea-1-2]: https://docs.rs/rug/~1.2/rug/ops/trait.NegAssign.html
[noa-1-2]: https://docs.rs/rug/~1.2/rug/ops/trait.NotAssign.html
[rat-1-2]: https://docs.rs/rug/~1.2/rug/struct.Rational.html
[rat-d-1-2]: https://docs.rs/rug/~1.2/rug/struct.Rational.html#method.dot
[rat-p-1-2]: https://docs.rs/rug/~1.2/rug/struct.Rational.html#method.product
[rat-s-1-2]: https://docs.rs/rug/~1.2/rug/struct.Rational.html#method.sum

Version 1.1.1 (2018-05-20)
==========================

  * Support for unstable [`i128`], [`u128`] and [`TryFrom`] was added in
    nightly.

Version 1.1.0 (2018-04-23)
==========================

  * Support for [`i128`] and [`u128`] conversions and comparisons was added,
    conditional on compiler support.
  * Conditional on compiler support, [`TryFrom`] conversions were implemented
    for conversions
      * from [`Integer`][int-1-1] values to integer primitives,
      * from floating-point primitives to [`Rational`][rat-1-1] numbers, and
      * from [`Float`][flo-1-1] values to [`Rational`][rat-1-1] numbers.
  * A new method <code>[Float][flo-1-1]::[get\_significand][flo-gs-1-1]</code>
    was added.
  * New methods <code>[Float][flo-1-1]::[u\_pow\_u][flo-upu-1-1]</code> and
    <code>[Float][flo-1-1]::[i\_pow\_u][flo-ipu-1-1]</code> were added.
  * New methods <code>[Integer][int-1-1]::[from\_digits][int-fd-1-1]</code>,
    <code>[Integer][int-1-1]::[to\_digits][int-td-1-1]</code>,
    <code>[Integer][int-1-1]::[assign\_digits][int-ad-1-1]</code>,
    <code>[Integer][int-1-1]::[write\_digits][int-wd-1-1]</code> and
    <code>[Integer][int-1-1]::[significant\_digits][int-sd-1-1]</code> were
    added, providing reading from and writing to slices of unsigned integer
    primitives.

[flo-1-1]: https://docs.rs/rug/~1.1/rug/struct.Float.html
[flo-gs-1-1]: https://docs.rs/rug/~1.1/rug/struct.Float.html#method.get_significand
[flo-ipu-1-1]: https://docs.rs/rug/~1.1/rug/struct.Float.html#method.i_pow_u
[flo-upu-1-1]: https://docs.rs/rug/~1.1/rug/struct.Float.html#method.u_pow_u
[int-1-1]: https://docs.rs/rug/~1.1/rug/struct.Integer.html
[int-ad-1-1]: https://docs.rs/rug/~1.1/rug/struct.Integer.html#method.assign_digits
[int-fd-1-1]: https://docs.rs/rug/~1.1/rug/struct.Integer.html#method.from_digits
[int-sd-1-1]: https://docs.rs/rug/~1.1/rug/struct.Integer.html#method.significant_digits
[int-td-1-1]: https://docs.rs/rug/~1.1/rug/struct.Integer.html#method.to_digits
[int-wd-1-1]: https://docs.rs/rug/~1.1/rug/struct.Integer.html#method.write_digits
[rat-1-1]: https://docs.rs/rug/~1.1/rug/struct.Rational.html

Version 1.0.2 (2018-04-09)
==========================

  * A bug in <code>[Integer][int-1-0]::[reserve][int-r-1-0]</code>, which was
    shrinking the capacity in some cases, was fixed.
  * The [gmp-mpfr-sys] dependency in [*Cargo.toml*] was fixed to use tilde
    instead of caret, since Rug uses internal implementation details.

Version 1.0.1 (2018-03-10)
==========================

  * A new method
    <code>[Integer][int-1-0]::[is\_power\_of\_two][int-ipot-1-0]</code> was
    added.
  * A new method <code>[Integer][int-1-0]::[signed\_bits][int-sb-1-0]</code> was
    added.
  * New methods
    <code>[Integer][int-1-0]::[secure\_pow\_mod][int-spm-1-0]</code>,
    <code>[Integer][int-1-0]::[secure\_pow\_mod\_mut][int-spmm-1-0]</code> and
    <code>[Integer][int-1-0]::[secure\_pow\_mod\_ref][int-spmr-1-0]</code> were
    added.
  * New methods <code>[Integer][int-1-0]::[div\_rem\_round][int-drr-1-0]</code>,
    <code>[Integer][int-1-0]::[div\_rem\_round\_mut][int-drrm-1-0]</code> and
    <code>[Integer][int-1-0]::[div\_rem\_round\_ref][int-drrr-1-0]</code> were
    added.
  * A new method <code>[Complex][com-1-0]::[eq0][com-e-1-0]</code> was added.
  * Documentation was improved.

Version 1.0.0 (2018-03-03)
==========================

  * The methods <code>[Integer][int-1-0]::[invert\_mut][int-im-1-0]</code> and
    <code>[Integer][int-1-0]::[pow\_mod\_mut][int-pmm-1-0]</code> were changed
    to return <code>[Result][`Result`]&lt;(), ()&gt;</code> instead of [`bool`].
  * The <code>[float][flom-1-0]::[Round][rou-1-0]</code>,
    <code>[float][flom-1-0]::[Constant][con-1-0]</code> and
    <code>[float][flom-1-0]::[Special][spe-1-0]</code> enums are now marked as
    non-exhaustive.
  * All deprecated items were removed.
  * Unsound blanket implementations constrained on <code>T
    where [SmallInteger][smi-1-0]: [Assign][ass-1-0]&lt;T&gt;</code> inside
    [`SmallRational`][smr-1-0] and on <code>T where [SmallFloat][smf-1-0]:
    [Assign][ass-1-0]&lt;T&gt;</code> inside [`SmallComplex`][smc-1-0] were
    removed.

[ass-1-0]: https://docs.rs/rug/~1.0/rug/trait.Assign.html
[com-1-0]: https://docs.rs/rug/~1.0/rug/struct.Complex.html
[com-e-1-0]: https://docs.rs/rug/~1.0/rug/struct.Complex.html#method.eq0
[con-1-0]: https://docs.rs/rug/~1.0/rug/float/enum.Constant.html
[flom-1-0]: https://docs.rs/rug/~1.0/rug/float/index.html
[int-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html
[int-drr-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.div_rem_round
[int-drrm-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.div_rem_round_mut
[int-drrr-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.div_rem_round_ref
[int-im-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.invert_mut
[int-ipot-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.is_power_of_two
[int-pmm-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.pow_mod_mut
[int-r-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.reserve
[int-sb-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.signed_bits
[int-spm-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.secure_pow_mod
[int-spmm-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.secure_pow_mod_mut
[int-spmr-1-0]: https://docs.rs/rug/~1.0/rug/struct.Integer.html#method.secure_pow_mod_ref
[rou-1-0]: https://docs.rs/rug/~1.0/rug/float/enum.Round.html
[smc-1-0]: https://docs.rs/rug/~1.0/rug/complex/struct.SmallComplex.html
[smf-1-0]: https://docs.rs/rug/~1.0/rug/float/struct.SmallFloat.html
[smi-1-0]: https://docs.rs/rug/~1.0/rug/integer/struct.SmallInteger.html
[smr-1-0]: https://docs.rs/rug/~1.0/rug/rational/struct.SmallRational.html
[spe-1-0]: https://docs.rs/rug/~1.0/rug/float/enum.Special.html

Version 0.10.0 (2018-02-17)
===========================

  * `Integer::invert_ref` and `Integer::pow_mod_ref` now return an `Option`, not
    an object that is assignable to `Result`.
  * `Float::random_bits` and `Complex::random_bits` are now assignable to the
    number values, not to `Result` objects.
  * `Rational::signum`, `Rational::trunc`, `Rational::ceil`, `Rational::floor`
    and `Rational::round` now return `Rational`.
  * `Complex::abs`, `Complex::arg` and `Complex::norm` now return `Complex`.
  * Remove `Default` implementations from all types of `Float` and `Complex`;
    now the precision always has to be specified.
  * Remove `Sum` and `Product` implementations for `Float` and `Complex`.
  * Remove `Clone` and `Copy` implementations from all incomplete computation
    types.
  * Revamp top-level crate documentation.
  * Add `Integer::parse` and `Rational::parse`, and deprecate `ValidInteger`,
    `ValidRational`, `valid_str_radix` methods, and `assign_str*` methods.
  * Add `Float::parse` and `Complex::parse`, and deprecate `ValidFloat`,
    `ValidComplex`, `from_str*` methods, `valid_str_radix` methods, and
    `assign_str*` methods.
  * Rename `Integer::gcd_coeffs*` methods to `Integer::gcd_cofactors*`.
  * `Integer::gcd_cofactors_ref` now supports computing only one cofactor.
  * Deprecate `Rational::to_integer` and `Rational::as_numer_denom`.
  * Deprecate `Rational::as_mut_numer_denom` and replace with
    `Rational::mutate_numer_denom`.
  * Deprecate `Complex::as_real_imag`.

Version 0.9.3 (2018-02-09)
==========================

  * Add `Integer::square` and `Rational::square` methods.
  * Add `Rational::cmp_abs` method.
  * Add `Float::sum` and `Complex::sum` methods.
  * Add `from_raw`, `into_raw`, `as_raw` and `as_raw_mut` to `RandState`.
  * Add `RandGen::boxed_clone` and `RandState::new_custom_boxed`, and thus
    support for cloning custom random generators.
  * Fix `Float::PartialOrd<f32>` (and `<f64>`) to return `None` when the
    primitive is NaN.

Version 0.9.2 (2018-01-12)
==========================

  * Require rustc version 1.18.0.
  * Require gmp-mpfr-sys version 1.1.0.
  * Deprecate most `assign_*` methods, and replace with static methods that
    return an assignable object.
  * Deprecate `Rational::copy_to_integer` method.
  * Add `Rational::assign_canonical` and `Rational::from_canonical` methods.
  * Add `Float::ln_u` method.
  * Add `Float::round_even` methods.
  * Add `Float::gamma_inc` methods.
  * Add `Float::random_normal` and `Float::random_exp` methods.
  * Deprecate `Float::assign_random_gaussian` methods.
  * Add `Complex::cmp_abs` method.
  * Add `Complex::root_of_unity` method.
  * Deprecate `SmallRational::from_canonicalized_*` methods and replace with
    `SmallRational::from_canonical` method.
  * Deprecate `SmallRational::assign_canonicalized_*` methods.
  * Add `as_nonreallocating_*` methods to `SmallInteger`, `SmallRational`,
    `SmallFloat` and `SmallComplex`.
  * Fix `SmallFloat::new` and `SmallComplex::new` to produce numbers with a
    precision of 53.
  * Deprecate and hide `float::Round::AwayFromZero`.
  * Add `Integer::signum`, `Rational::signum` and `Float::signum` methods.
  * Add missing conversion to/from and comparisons to primitive types.
  * Add `from_raw`, `into_raw`, `as_raw` and `as_raw_mut` to `Integer`,
    `Rational`, `Float` and `Complex`.
  * Add `Float::classify` method.
  * Add `Float::mul_add`, `Float::mul_sub`, `Float::mul_add_mul` and
    `Float::mul_sub_mul` methods.
  * Add `Complex::mul_add` and `Complex::mul_sub` methods.

Version 0.9.1 (2017-11-27)
==========================

  * Implement mathematical operations where operands include references to
    primitives.
  * Remove undefined behaviour: replace `mem::swap(&mut src, &mut
    uninitialized)` with `ptr::copy_nonoverlapping(&src, &mut uninitialized,
    1)`.

Version 0.9.0 (2017-11-16)
==========================

  * Move `rug::float::AssignRound` to `rug::ops::AssignRound`.
  * `OrdFloat` now orders +NaN above +∞, while &minus;NaN is still below
    &minus;∞.
  * Change `Float::subnormalize` methods to require explicit minimum normal
    exponent.
  * Add `Float::subnormalize_ieee` methods to deduce exponent range from
    precision, like old `Float::subnormalize`. The new method also supports all
    IEEE 754-2008 precisions corresponding to k storage bits where k ≥ 128 and k
    is a multiple of 32.
  * Deprecate `Rational::fract` methods and replace with `Rational::rem_trunc`
    methods.
  * Add `Rational::rem_ceil` and `Rational::rem_floor` methods.
  * Add `Rational::rem_round` and `Rational::fract_round` methods.
  * Add `Float::next_toward`, `Float::next_up` and `Float::next_down`.
  * Add optional serde support.

Version 0.8.0 (2017-10-26)
==========================

  * Rename `Integer::sign`, `Rational::sign` and `Float::sign` methods as
    `Integer::cmp0`, `Rational::cmp0` and `Float::cmp0`.
  * Rename `Float::pos_diff` as `Float::positive_diff`.
  * Move `rug::AssignRound` to `rug::float::AssignRound`.
  * Add `Integer::clamp`, `Rational::clamp` and `Float::clamp` methods.
  * Add `Integer::div_rem_ceil`, `Integer::div_rem_floor` and
    `Integer::div_rem_euc` methods.
  * Add `DivRounding`, `DivRoundingAssign` and `DivRoundingFrom` traits, and
    their `Rem` counterparts, and implement them for `Integer`, and for
    combinations of `Integer` with `i32` or `u32`.
  * Add `Rational::fract_floor` and `Rational::fract_ceil` methods.

Version 0.7.0 (2017-09-30)
==========================

  * Fix swapped `Float::sin_cos_mut` and `Float::sin_cos_round`,
    `Float::sinh_cosh_mut` and `Float::sinh_cosh_round`, and
    `Float::trunc_fract_mut` and `Float::trunc_fract_round`.
  * Fix `Float::to_f32_round`.
  * Now `Float::subnormalize` only works for precisions defined in IEEE 754.
  * Add `Integer::gcd_coeffs` methods.

Version 0.6.0 (2017-08-09)
==========================

  * Requires rustc version 1.17.0.
  * Rename `Float::abs_diff` as `Float::pos_diff`.
  * Replace `Float::get_sign` with `Float::is_sign_positive` and
    `Float::is_sign_negative`.
  * Add various `as_neg`, `as_abs` and `as_conj` methods.
  * Add `OrdFloat` and `OrdComplex` for complete ordering.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*az* crate]: https://crates.io/crates/az
[*num-integer* crate]: https://crates.io/crates/num-integer
[*num-traits* crate]: https://crates.io/crates/num-traits
[NegOutput]: https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html#associatedtype.Output
[`AsMut`]: https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html
[`AsRef`]: https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html
[`Clone`]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html
[`Debug`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html
[`Default`]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html
[`Deref`]: https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html
[`Eq`]: https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`LowerExp`]: https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html
[`MaybeUninit`]: https://doc.rust-lang.org/nightly/core/mem/union.MaybeUninit.html
[`Neg`]: https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html
[`Option`]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html
[`PartialEq`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html
[`PartialOrd`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html
[`Product`]: https://doc.rust-lang.org/nightly/core/iter/trait.Product.html
[`RemAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.RemAssign.html
[`Rem`]: https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html
[`Result`]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html
[`Send`]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html
[`ShlAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html
[`Shl`]: https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html
[`ShrAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.ShrAssign.html
[`Shr`]: https://doc.rust-lang.org/nightly/core/ops/trait.Shr.html
[`Sum`]: https://doc.rust-lang.org/nightly/core/iter/trait.Sum.html
[`Sync`]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html
[`TryFrom`]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html
[`UpperExp`]: https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html
[`bool`]: https://doc.rust-lang.org/nightly/core/primitive.bool.html
[`deref`]: https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html#tymethod.deref
[`f32`]: https://doc.rust-lang.org/nightly/core/primitive.f32.html
[`f64`]: https://doc.rust-lang.org/nightly/core/primitive.f64.html
[`i128`]: https://doc.rust-lang.org/nightly/core/primitive.i128.html
[`i16`]: https://doc.rust-lang.org/nightly/core/primitive.i16.html
[`i32`]: https://doc.rust-lang.org/nightly/core/primitive.i32.html
[`i64`]: https://doc.rust-lang.org/nightly/core/primitive.i64.html
[`i8`]: https://doc.rust-lang.org/nightly/core/primitive.i8.html
[`isize`]: https://doc.rust-lang.org/nightly/core/primitive.isize.html
[`mem`]: https://doc.rust-lang.org/nightly/core/mem/index.html
[`u128`]: https://doc.rust-lang.org/nightly/core/primitive.u128.html
[`u16`]: https://doc.rust-lang.org/nightly/core/primitive.u16.html
[`u32`]: https://doc.rust-lang.org/nightly/core/primitive.u32.html
[`u64`]: https://doc.rust-lang.org/nightly/core/primitive.u64.html
[`u8`]: https://doc.rust-lang.org/nightly/core/primitive.u8.html
[`uninitialized`]: https://doc.rust-lang.org/nightly/core/mem/fn.uninitialized.html
[`usize`]: https://doc.rust-lang.org/nightly/core/primitive.usize.html
[gmp-mpfr-sys]: https://crates.io/crates/gmp-mpfr-sys
[slice]: https://doc.rust-lang.org/nightly/core/primitive.slice.html
[tuple]: https://doc.rust-lang.org/nightly/core/primitive.tuple.html
