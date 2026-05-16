-- SPDX-License-Identifier: Apache-2.0
-- Copyright 2025-2026 Ian Farquharson
import Lake
open Lake DSL

package scc_kernel_fortress_formal where
  -- Pure Lean 4 bridge for the implemented v1 acceptance model.

@[default_target]
lean_lib SCC where
  roots := #[`SCC.KernelFortress.GateSoundness, `SCC.KernelFortress.PipelineWitness]
