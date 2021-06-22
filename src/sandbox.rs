use crate::portfolio_manager::Portfolio;
use crate::signal_builder::{Strategy};
use crate::types::DateTime;

struct Sandbox;

/// Encapsulates the application of a trading strategy over a specific period.
struct TimedStrategy {
    strategy: Strategy,
    from: DateTime,
    to: DateTime,
}

/// Add backtesting validation metrics here.
struct BacktestReport;

impl Sandbox {
    /// Run a set of strategies (each given a specific time period of activity) against an initial
    /// portfolio and generate a detailed result.
    fn backtest(_initial_portfolios: Vec<Portfolio>, _strategies: Vec<Strategy>, _from: DateTime, _to: DateTime) -> BacktestReport {
        todo!()
    }
}
