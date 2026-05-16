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

def ModeSound (_b : Bundle) : Prop := True

def GateProp (env : Env) (b : Bundle) (prev next : State) : Prop :=
  Required b /\
  ProtectedIsolation prev next /\
  HaltAbsorbing prev next /\
  RiskToHalt env next /\
  LineageContinues prev next /\
  ModeSound b

instance instDecidableGateProp (env : Env) (b : Bundle) (prev next : State) :
    Decidable (GateProp env b prev next) := by
  unfold GateProp Required ProtectedIsolation HaltAbsorbing RiskToHalt LineageContinues ModeSound
  infer_instance

def GateBool (env : Env) (b : Bundle) (prev next : State) : Bool :=
  decide (GateProp env b prev next)

def Check (env : Env) (b : Bundle) (prev next : State) : Decision :=
  if GateBool env b prev next then Decision.accept else Decision.reject GateError.refinementViolation

theorem gateBool_sound (env : Env) (b : Bundle) (prev next : State) :
    GateBool env b prev next = true -> GateProp env b prev next := by
  intro h
  unfold GateBool at h
  exact of_decide_eq_true h

theorem check_accept_sound (env : Env) (b : Bundle) (prev next : State) :
    Check env b prev next = Decision.accept -> GateProp env b prev next := by
  intro h
  unfold Check at h
  by_cases hp : GateBool env b prev next = true
  · exact gateBool_sound env b prev next hp
  · simp [hp] at h

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

end SCC.KernelFortress
