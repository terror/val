/* tfr_div -- test file for mpc_fr_div.

Copyright (C) 2008, 2013 INRIA

This file is part of GNU MPC.

GNU MPC is free software; you can redistribute it and/or modify it under
the terms of the GNU Lesser General Public License as published by the
Free Software Foundation; either version 3 of the License, or (at your
option) any later version.

GNU MPC is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for
more details.

You should have received a copy of the GNU Lesser General Public License
along with this program. If not, see http://www.gnu.org/licenses/ .
*/

#include <math.h>

#include "mpc-tests.h"

#define MPC_FUNCTION_CALL                                               \
  P[0].mpc_inex = mpc_fr_div (P[1].mpc, P[2].mpfr, P[3].mpc, P[4].mpc_rnd)
#define MPC_FUNCTION_CALL_REUSE_OP2                                     \
  P[0].mpc_inex = mpc_fr_div (P[1].mpc, P[2].mpfr, P[1].mpc, P[4].mpc_rnd)

#include "data_check.tpl"
#include "tgeneric.tpl"

static void
check_divby0_exc (void)
{
  mpc_t z;
  mpfr_t n;
  double nums[] = {0.0, NAN, INFINITY, -0.0, 1.0, -2.5};

  mpc_init2 (z, 53);
  mpfr_init2 (n, 53);
  for (size_t i = 0; i < sizeof(nums)/sizeof(double); i++) {
    mpc_set_d_d (z, 0.0, 0.0, MPC_RNDNN);
    mpfr_set_d (n, nums[i], MPFR_RNDN);
    mpfr_clear_flags ();
    mpc_fr_div (z, n, z, MPC_RNDNN);
    if (mpfr_regular_p (n) && !mpfr_divby0_p ()) {
      printf ("Missing division-by-zero exception for n=%la\n", nums[i]);
      exit (1);
    }
    if (mpfr_inf_p (n) && mpfr_divby0_p ()) {
      printf ("division-by-zero exception for n=%la\n", nums[i]);
      exit (1);
    }
    if ((mpfr_nan_p (n) || mpfr_zero_p (n))) {
      if (mpfr_divby0_p ()) {
        printf ("division-by-zero exception for n=%la\n", nums[i]);
        exit (1);
      }
      if (!mpfr_nanflag_p ()) {
        printf ("Missing nanflag exception for n=%la\n", nums[i]);
        exit (1);
      }
    }
  }
  mpfr_clear_flags ();
  mpc_clear (z);
  mpfr_clear (n);
}

/* check that mpc_fr_div does not change the precision of its result,
   see https://sympa.inria.fr/sympa/arc/mpc-discuss/2026-03/msg00020.html */
static void
bug20260331 (void)
{
  mpc_t x;
  mpfr_t y;
  const mpfr_prec_t prec = 160;
  mpc_init2 (x, prec);
  mpfr_init2 (y, prec);
  mpc_set_ui (x, 5, MPC_RNDNN);
  mpfr_set_ui (y, 1, MPFR_RNDN);
  mpc_fr_div (x, y, x, MPC_RNDNN);
  if (mpfr_get_prec (mpc_realref (x)) != prec) {
    printf ("Error, precision of real part changed\n");
    exit (1);
  }
  if (mpfr_get_prec (mpc_imagref (x)) != prec) {
    printf ("Error, precision of imaginary part changed\n");
    exit (1);
  }
  mpc_clear (x);
  mpfr_clear (y);
}

int
main (void)
{
  test_start();

  bug20260331 ();

  check_divby0_exc ();

  data_check_template ("fr_div.dsc", "fr_div.dat");

  tgeneric_template ("fr_div.dsc", 2, 1024, 7, 65535);

  test_end ();

  return 0;
}
