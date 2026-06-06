/* timer.c - helper functions for measuring elapsed cpu time

Copyright (C) 2009, 2010, 2021, 2024 INRIA

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

/*****************************************************************************/

void mpc_timer_reset (mpc_timer_t t)

{
   t->elapsed = 0;
}

/*****************************************************************************/

void mpc_timer_continue (mpc_timer_t t)

{
   t->time_old = clock ();
}

/*****************************************************************************/

void mpc_timer_start (mpc_timer_t t)

{
   mpc_timer_reset (t);
   mpc_timer_continue (t);
}

/*****************************************************************************/

void mpc_timer_stop (mpc_timer_t t)

{
   clock_t time_new;

   time_new = clock ();
   t->elapsed += ((double) (time_new - t->time_old)) / CLOCKS_PER_SEC;
}

/*****************************************************************************/

double mpc_timer_get (mpc_timer_t t)

{
   return t->elapsed;
}

/*****************************************************************************/

