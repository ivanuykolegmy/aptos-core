// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use diem_transactional_test_harness::run_dpn_test;

datatest_stable::harness!(run_dpn_test, "DPN/transactional-tests", r".*\.(mvir|move)$");