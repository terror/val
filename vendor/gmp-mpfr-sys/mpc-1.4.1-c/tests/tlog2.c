/* tlog2 -- test file for mpc_log2.

Copyright (C) 2012, 2013, 2025 INRIA

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
  P[0].mpc_inex = mpc_log2 (P[1].mpc, P[2].mpc, P[3].mpc_rnd)
#define MPC_FUNCTION_CALL_REUSE_OP1                                     \
  P[0].mpc_inex = mpc_log2 (P[1].mpc, P[1].mpc, P[3].mpc_rnd)

#include "data_check.tpl"
#include "tgeneric.tpl"

static void
check_divby0_exc (void)
{
  mpc_t z;
  struct {
    double re, im;
  } data[] = {{+0.0, +0.0},
              {+0.0, -0.0},
              {-0.0, +0.0},
              {-0.0, -0.0}};

  mpc_init2 (z, 53);
  for (size_t i = 0; i < 4; i++) {
    mpc_set_d_d (z, data[i].re, data[i].im, MPC_RNDNN);
    mpfr_clear_flags ();
    mpc_log2 (z, z, MPC_RNDNN);
    if (!mpfr_divby0_p ()) {
      printf ("Missing division-by-zero exception for (%la,%la)\n",
              data[i].re, data[i].im);
      exit (1);
    }
  }
  mpfr_clear_flags ();
  mpc_clear (z);
}

int
main (void)
{
  check_divby0_exc ();

  test_start ();

  data_check_template ("log2.dsc", "log2.dat");

  tgeneric_template ("log2.dsc", 2, 512, 7, 128);

  test_end ();

  return 0;
}
