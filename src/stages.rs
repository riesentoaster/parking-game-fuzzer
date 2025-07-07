//! Custom stages for optimized solving of [`parking_game`] puzzles.

use crate::feedbacks::{FinalStateMetadata, ViewMetadata};
use crate::input::PGInput;
use crate::observers::PGObserverTuple;
use libafl::executors::{ExitKind, HasObservers};
use libafl::feedbacks::Feedback;
use libafl::observers::ObserversTuple;
use libafl::schedulers::Scheduler;
use libafl::stages::{Restartable, Stage};
use libafl::state::{HasCurrentTestcase, HasExecutions};
use libafl::{ExecutionProcessor, HasFeedback, HasMetadata, HasObjective, HasScheduler};
use libafl_bolts::Error;
use parking_game::{BoardValue, State};
use std::marker::PhantomData;
use std::ops::Deref;

/// A stage implementation which exhausts the mutation space rather than randomly selecting
/// mutations.
pub struct PGMutationStage<T> {
    phantom: PhantomData<T>,
}

impl<T> PGMutationStage<T> {
    /// Create a new mutation stage for the provided initial state.
    pub fn new(_init: &State<T>) -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<S, T> Restartable<S> for PGMutationStage<T> {
    fn should_restart(&mut self, _state: &mut S) -> Result<bool, Error> {
        Ok(true)
    }

    fn clear_progress(&mut self, _state: &mut S) -> Result<(), Error> {
        Ok(())
    }
}

impl<E, EM, S, T, Z> Stage<E, EM, S, Z> for PGMutationStage<T>
where
    // lots of bounds here!
    // this gives you a sense of just how many bounds you might need when doing complicated things
    // review each of the bounds here; you will need them all
    E: HasObservers,
    E::Observers: PGObserverTuple<T> + ObserversTuple<PGInput, S>,
    S: HasCurrentTestcase<PGInput> + HasExecutions,
    T: BoardValue,
    Z: HasFeedback
        + HasObjective
        + HasScheduler<PGInput, S>
        + ExecutionProcessor<EM, PGInput, E::Observers, S>,
    Z::Feedback: Feedback<EM, PGInput, E::Observers, S>,
    Z::Objective: Feedback<EM, PGInput, E::Observers, S>,
{
    fn perform(
        &mut self,
        fuzzer: &mut Z,
        executor: &mut E,
        state: &mut S,
        manager: &mut EM,
    ) -> Result<(), Error> {
        // TODO(pt.4) load the testcase, its view and final state metadata, and input, then drop it

        // TODO(pt.4) record the original number of the moves

        // TODO(pt.4) now, the hard part: efficiently enumerating the mutation space
        //  - here, we are going to enumerate all potential mutations based on the view data:
        //  - clone the state from the metadata
        //  - make a board view over it
        //  - for each car and its views
        //    - for each view
        //      - if that view has a non-zero distance
        //        - add a move in that direction to the input
        //        - pre_exec the observer tuple
        //        - increment the execution counts
        //        - apply the car shift to the board in the view's direction
        //        - final_board the observer tuple
        //        - post_exec the observer tuple
        //        - invoke the on_evaluation callback of the fuzzer's scheduler
        //        - evaluate the execution with fuzzer
        //        - if the execution was a solution, return from the stage
        //        - truncate the input to its original size
        //        - undo the car shift by applying a shift to the same car in the negative direction

        Ok(())
    }
}
