use crate::signal_processor::DataCollectorPoint;

/// A signal that will be triggered depending on incoming market state.
///
/// Note: this is a simplistic stub. More complex predicates will likely be necessary (e.g. examine
/// moving averages, which this cannot do because it looks only at one point).
pub type Signal = fn(DataCollectorPoint) -> bool;

/// A signal to be triggered when certain market conditions arise.
pub struct Strategy {
    signal: Signal,
    actions: Vec<Action>,
}

/// A trade action to be taken (e.g. buy/sell).
struct Action {
    action_type: ActionType,
    amount: u32,
}

/// The type of an action.
enum ActionType {
    BUY,
    SELL,
// Others are possible (depends on the domain).
}
