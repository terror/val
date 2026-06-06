/* tdiv_fr.c -- test file for mpc_div_fr.

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

#include "mpc-tests.h"

#define MPC_FUNCTION_CALL                                               \
  P[0].mpc_inex = mpc_div_fr (P[1].mpc, P[2].mpc, P[3].mpfr, P[4].mpc_rnd)
#define MPC_FUNCTION_CALL_REUSE_OP1                                     \
  P[0].mpc_inex = mpc_div_fr (P[1].mpc, P[1].mpc, P[3].mpfr, P[4].mpc_rnd)

#include "data_check.tpl"
#include "tgeneric.tpl"

static void
check_divby0_exc (void)
{
  mpc_t z;
  mpfr_t d;

  mpc_init2 (z, 53);
  mpfr_init2 (d, 53);
  mpfr_set_d (d, 0.0, MPFR_RNDN);
  mpc_set_d_d (z, 1.0, 0.0, MPC_RNDNN);
  mpfr_clear_flags ();
  mpc_div_fr (z, z, d, MPC_RNDNN);
  if (!mpfr_divby0_p ()) {
    printf ("Missing division-by-zero exception\n");
    exit (1);
  }
  mpfr_clear_flags ();
  mpc_clear (z);
  mpfr_clear (d);
}

int
main (void)
{
  test_start ();

  check_divby0_exc ();

  data_check_template ("div_fr.dsc", "div_fr.dat");

  tgeneric_template ("div_fr.dsc", 2, 1024, 7, 1024);

  test_end ();

  return 0;
}
