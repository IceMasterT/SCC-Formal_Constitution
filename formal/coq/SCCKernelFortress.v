(* SPDX-License-Identifier: Apache-2.0 *)
(* Copyright 2025-2026 Ian Farquharson *)
Require Import Coq.Bool.Bool.
Require Import Coq.Arith.Arith.
Require Import Coq.micromega.Lia.

Record bundle := mkBundle {
  dom : bool;
  inv : bool;
  identity : bool;
  governance : bool;
  risk : bool;
  audit : bool;
  isolation : bool;
  refinement : bool
}.

Definition required_bool (b : bundle) : bool :=
  dom b && inv b && identity b && governance b && risk b && audit b && isolation b && refinement b.

Definition required_prop (b : bundle) : Prop :=
  dom b = true /\
  inv b = true /\
  identity b = true /\
  governance b = true /\
  risk b = true /\
  audit b = true /\
  isolation b = true /\
  refinement b = true.

Theorem required_bool_sound : forall b, required_bool b = true -> required_prop b.
Proof.
  intros b H.
  unfold required_bool in H.
  repeat rewrite Bool.andb_true_iff in H.
  destruct H as [[[[[[[Hdom Hinv] Hidentity] Hgovernance] Hrisk] Haudit] Hisolation] Hrefinement].
  repeat split; assumption.
Qed.

Theorem quorum_overlap_lower :
  forall f n q overlap,
    n = 3 * f + 1 ->
    q = 2 * f + 1 ->
    q + q <= n + overlap ->
    overlap >= f + 1.
Proof.
  intros f n q overlap Hn Hq Hunion.
  subst n.
  subst q.
  lia.
Qed.

Theorem quorum_overlap_has_honest_node :
  forall f overlap,
    overlap >= f + 1 -> overlap > f.
Proof.
  intros f overlap H.
  lia.
Qed.
