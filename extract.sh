#!/bin/bash

set -v
llvm-extract -S --func=cint_wrap --func=c2s_cart_1e --func=CINTlrys_laguerre --recursive --rfunc="enzyme_opt_helper_*" out.ll -o mwe.ll