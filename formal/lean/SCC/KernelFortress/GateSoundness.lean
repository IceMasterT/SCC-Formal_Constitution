-- SPDX-License-Identifier: Apache-2.0
-- Copyright 2025-2026 Ian Farquharson
import Std

namespace SCC.KernelFortress

inductive StepMode where
  | exact
  | stutter
  | safeRefined
  deriving DecidableEq, Repr

inductive GateError where
  | missingRequiredField
  | protectedMutation
  | haltViolation
  | riskThresholdBreach
  | lineageViolation
  | refinementViolation
  deriving DecidableEq, Repr

inductive Decision where
  | accept
  | reject (e : GateError)
  deriving DecidableEq, Repr

structure Env where
  threshold : Nat
  deriving DecidableEq, Repr

structure State where
  protectedRoot : Nat
  lineageRoot : Nat
  risk : Nat
  failureMode : Nat
  deriving DecidableEq, Repr

structure Bundle where
  dom : Bool
  inv : Bool
  identity : Bool
  governance : Bool
  risk : Bool
  audit : Bool
  isolation : Bool
  refinement : Bool
  declaredMode : StepMode
  deriving DecidableEq, Repr

def Required (b : Bundle) : Prop :=
  b.dom = true /\
  b.inv = true /\
  b.identity = true /\
  b.governance = true /\
  b.risk = true /\
  b.audit = true /\
  b.isolation = true /\
  b.refinement = true

def ProtectedIsolation (prev next : State) : Prop :=
  next.protectedRoot = prev.protectedRoot

def HaltAbsorbing (prev next : State) : Prop :=
  prev.failureMode = 3 -> next.failureMode = 3

def RiskToHalt (env : Env) (next : State) : Prop :=
  next.risk >= env.threshold -> next.failureMode = 3

def LineageContinues (prev next : State) : Prop :=
  next.lineageRoot = prev.lineageRoot + 1

/-- Normalized projection used for step classification: the administrative
lineage coordinate is dropped, matching the implementation's normalized state. -/
structure Norm where
  protectedRoot : Nat
  risk : Nat
  failureMode : Nat
  deriving DecidableEq, Repr

def normalize (s : State) : Norm :=
  { protectedRoot := s.protectedRoot, risk := s.risk, failureMode := s.failureMode }

/-- Induced abstract successor in the small model: risk is carried forward and
the failure mode escalates to halt when risk meets the descriptor threshold. -/
def expectedNorm (env : Env) (prev : State) : Norm :=
  { protectedRoot := prev.protectedRoot
    risk := prev.risk
    failureMode := if prev.risk >= env.threshold then 3 else prev.failureMode }

/-- The declared step mode must match its classification obligation:
`exact` reaches the induced successor, `stutter` preserves the normalized
state, and `safeRefined` is protected-preserving, risk-nonincreasing, and
severity-nondecreasing relative to the induced successor. -/
def ModeSound (env : Env) (b : Bundle) (prev next : State) : Prop :=
  (b.declaredMode = StepMode.exact -> normalize next = expectedNorm env prev) /\
  (b.declaredMode = StepMode.stutter -> normalize next = normalize prev) /\
  (b.declaredMode = StepMode.safeRefined ->
    next.protectedRoot = prev.protectedRoot /\
    next.risk <= (expectedNorm env prev).risk /\
    (expectedNorm env prev).failureMode <= next.failureMode)

instance instDecidableRequired (b : Bundle) : Decidable (Required b) := by
  unfold Required
  infer_instance

instance instDecidableProtectedIsolation (prev next : State) :
    Decidable (ProtectedIsolation prev next) := by
  unfold ProtectedIsolation
  infer_instance

instance instDecidableHaltAbsorbing (prev next : State) :
    Decidable (HaltAbsorbing prev next) := by
  unfold HaltAbsorbing
  infer_instance

instance instDecidableRiskToHalt (env : Env) (next : State) :
    Decidable (RiskToHalt env next) := by
  unfold RiskToHalt
  infer_instance

instance instDecidableLineageContinues (prev next : State) :
    Decidable (LineageContinues prev next) := by
  unfold LineageContinues
  infer_instance

instance instDecidableModeSound (env : Env) (b : Bundle) (prev next : State) :
    Decidable (ModeSound env b prev next) := by
  unfold ModeSound
  infer_instance

def GateProp (env : Env) (b : Bundle) (prev next : State) : Prop :=
  Required b /\
  ProtectedIsolation prev next /\
  HaltAbsorbing prev next /\
  RiskToHalt env next /\
  LineageContinues prev next /\
  ModeSound env b prev next

instance instDecidableGateProp (env : Env) (b : Bundle) (prev next : State) :
    Decidable (GateProp env b prev next) := by
  unfold GateProp Required ProtectedIsolation HaltAbsorbing RiskToHalt LineageContinues ModeSound
  infer_instance

def GateBool (env : Env) (b : Bundle) (prev next : State) : Bool :=
  decide (GateProp env b prev next)

/-- First-failure pipeline mirroring the implementation's gate order for the
modeled subset: each rejection carries the name of the first failed gate. -/
def firstFailure (env : Env) (b : Bundle) (prev next : State) : Option GateError :=
  if Required b then
    if ProtectedIsolation prev next then
      if HaltAbsorbing prev next then
        if RiskToHalt env next then
          if LineageContinues prev next then
            if ModeSound env b prev next then none
            else some GateError.refinementViolation
          else some GateError.lineageViolation
        else some GateError.riskThresholdBreach
      else some GateError.haltViolation
    else some GateError.protectedMutation
  else some GateError.missingRequiredField

def Check (env : Env) (b : Bundle) (prev next : State) : Decision :=
  match firstFailure env b prev next with
  | none => Decision.accept
  | some e => Decision.reject e

theorem gateBool_sound (env : Env) (b : Bundle) (prev next : State) :
    GateBool env b prev next = true -> GateProp env b prev next := by
  intro h
  unfold GateBool at h
  exact of_decide_eq_true h

theorem firstFailure_none_sound (env : Env) (b : Bundle) (prev next : State) :
    firstFailure env b prev next = none -> GateProp env b prev next := by
  intro h
  by_cases h1 : Required b
  · by_cases h2 : ProtectedIsolation prev next
    · by_cases h3 : HaltAbsorbing prev next
      · by_cases h4 : RiskToHalt env next
        · by_cases h5 : LineageContinues prev next
          · by_cases h6 : ModeSound env b prev next
            · exact ⟨h1, h2, h3, h4, h5, h6⟩
            · simp [firstFailure, h1, h2, h3, h4, h5, h6] at h
          · simp [firstFailure, h1, h2, h3, h4, h5] at h
        · simp [firstFailure, h1, h2, h3, h4] at h
      · simp [firstFailure, h1, h2, h3] at h
    · simp [firstFailure, h1, h2] at h
  · simp [firstFailure, h1] at h

theorem check_accept_sound (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> GateProp env b prev next := by
  intro h
  unfold Check at h
  split at h
  · exact firstFailure_none_sound env b prev next (by assumption)
  · exact Decision.noConfusion h

theorem check_accept_iff_gateBool (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept <-> GateBool env b prev next = true := by
  constructor
  · intro h
    exact decide_eq_true (check_accept_sound env b prev next h)
  · intro h
    have hg := gateBool_sound env b prev next h
    unfold Check
    by_cases h1 : Required b
    · by_cases h2 : ProtectedIsolation prev next
      · by_cases h3 : HaltAbsorbing prev next
        · by_cases h4 : RiskToHalt env next
          · by_cases h5 : LineageContinues prev next
            · by_cases h6 : ModeSound env b prev next
              · simp [firstFailure, h1, h2, h3, h4, h5, h6]
              · exact absurd hg.2.2.2.2.2 h6
            · exact absurd hg.2.2.2.2.1 h5
          · exact absurd hg.2.2.2.1 h4
        · exact absurd hg.2.2.1 h3
      · exact absurd hg.2.1 h2
    · exact absurd hg.1 h1

/-- A `protectedMutation` rejection names its gate truthfully: the required
booleans passed and protected isolation is the first failed obligation. -/
theorem reject_protected_names_violation (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.reject GateError.protectedMutation ->
    Required b /\ ¬ ProtectedIsolation prev next := by
  intro h
  by_cases h1 : Required b
  · by_cases h2 : ProtectedIsolation prev next
    · by_cases h3 : HaltAbsorbing prev next
      · by_cases h4 : RiskToHalt env next
        · by_cases h5 : LineageContinues prev next
          · by_cases h6 : ModeSound env b prev next
            · simp [Check, firstFailure, h1, h2, h3, h4, h5, h6] at h
            · simp [Check, firstFailure, h1, h2, h3, h4, h5, h6] at h
          · simp [Check, firstFailure, h1, h2, h3, h4, h5] at h
        · simp [Check, firstFailure, h1, h2, h3, h4] at h
      · simp [Check, firstFailure, h1, h2, h3] at h
    · exact ⟨h1, h2⟩
  · simp [Check, firstFailure, h1] at h

/-- A `refinementViolation` rejection names its gate truthfully: every earlier
obligation passed and the declared-mode obligation is the one that failed. -/
theorem reject_refinement_names_violation (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.reject GateError.refinementViolation ->
    ¬ ModeSound env b prev next := by
  intro h
  by_cases h1 : Required b
  · by_cases h2 : ProtectedIsolation prev next
    · by_cases h3 : HaltAbsorbing prev next
      · by_cases h4 : RiskToHalt env next
        · by_cases h5 : LineageContinues prev next
          · by_cases h6 : ModeSound env b prev next
            · simp [Check, firstFailure, h1, h2, h3, h4, h5, h6] at h
            · exact h6
          · simp [Check, firstFailure, h1, h2, h3, h4, h5] at h
        · simp [Check, firstFailure, h1, h2, h3, h4] at h
      · simp [Check, firstFailure, h1, h2, h3] at h
    · simp [Check, firstFailure, h1, h2] at h
  · simp [Check, firstFailure, h1] at h

theorem accepted_preserves_protected (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> next.protectedRoot = prev.protectedRoot := by
  intro h
  have hg := check_accept_sound env b prev next h
  exact hg.2.1

theorem accepted_extends_lineage (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> next.lineageRoot = prev.lineageRoot + 1 := by
  intro h
  have hg := check_accept_sound env b prev next h
  exact hg.2.2.2.2.1

theorem accepted_respects_halt (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> prev.failureMode = 3 -> next.failureMode = 3 := by
  intro h
  have hg := check_accept_sound env b prev next h
  exact hg.2.2.1

theorem accepted_respects_risk_threshold (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> next.risk >= env.threshold -> next.failureMode = 3 := by
  intro h
  have hg := check_accept_sound env b prev next h
  exact hg.2.2.2.1

theorem accepted_exact_matches_successor (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> b.declaredMode = StepMode.exact ->
    normalize next = expectedNorm env prev := by
  intro h hm
  have hg := check_accept_sound env b prev next h
  exact hg.2.2.2.2.2.1 hm

theorem accepted_stutter_preserves_normalized (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> b.declaredMode = StepMode.stutter ->
    normalize next = normalize prev := by
  intro h hm
  have hg := check_accept_sound env b prev next h
  exact hg.2.2.2.2.2.2.1 hm

theorem accepted_safe_refined_is_risk_nonincreasing (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> b.declaredMode = StepMode.safeRefined ->
    next.risk <= prev.risk := by
  intro h hm
  have hg := check_accept_sound env b prev next h
  exact (hg.2.2.2.2.2.2.2 hm).2.1

theorem accepted_safe_refined_is_severity_nondecreasing (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> b.declaredMode = StepMode.safeRefined ->
    (expectedNorm env prev).failureMode <= next.failureMode := by
  intro h hm
  have hg := check_accept_sound env b prev next h
  exact (hg.2.2.2.2.2.2.2 hm).2.2

end SCC.KernelFortress
