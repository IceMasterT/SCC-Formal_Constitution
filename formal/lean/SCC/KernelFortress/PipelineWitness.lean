-- SPDX-License-Identifier: Apache-2.0
-- Copyright 2025-2026 Ian Farquharson
import SCC.KernelFortress.GateSoundness

namespace SCC.KernelFortress

structure StageWitness where
  intakeTyped : Bool
  canonicalEvent : Bool
  domainAdmissible : Bool
  protectedIsolation : Bool
  governanceRiskHalt : Bool
  auditLineage : Bool
  refinementClassified : Bool
  deriving DecidableEq, Repr

def StageWitnessProp (w : StageWitness) : Prop :=
  w.intakeTyped = true /\
  w.canonicalEvent = true /\
  w.domainAdmissible = true /\
  w.protectedIsolation = true /\
  w.governanceRiskHalt = true /\
  w.auditLineage = true /\
  w.refinementClassified = true

instance instDecidableStageWitnessProp (w : StageWitness) : Decidable (StageWitnessProp w) := by
  unfold StageWitnessProp
  infer_instance

def StageWitnessBool (w : StageWitness) : Bool :=
  decide (StageWitnessProp w)

def PipelineAccepts (env : Env) (b : Bundle) (prev next : State) (w : StageWitness) : Bool :=
  GateBool env b prev next && StageWitnessBool w

theorem stageWitnessBool_sound (w : StageWitness) :
    StageWitnessBool w = true -> StageWitnessProp w := by
  intro h
  unfold StageWitnessBool at h
  exact of_decide_eq_true h

theorem pipeline_accepts_sound (env : Env) (b : Bundle) (prev next : State) (w : StageWitness) :
    PipelineAccepts env b prev next w = true -> GateProp env b prev next /\ StageWitnessProp w := by
  intro h
  have hpair : GateBool env b prev next = true /\ StageWitnessBool w = true := by
    simpa [PipelineAccepts] using h
  exact And.intro (gateBool_sound env b prev next hpair.left) (stageWitnessBool_sound w hpair.right)

end SCC.KernelFortress
