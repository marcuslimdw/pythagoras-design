use crate::types::{CalculablePNL, CalculableTotalValue, DateTime, MoneyType, PositionSize};

/// Manages portfolios for the entire trading system.
pub struct PortfolioManager {
    active_portfolios: Vec<Portfolio>,
}

/// A portfolio for a given exchange (can be global)
pub struct Portfolio {
    exchange: Exchange,
    positions: Vec<Position>,
}

/// A cryptocurrency position.
struct Position {
    instrument: Instrument,
    size: PositionSize,
}

/// A cryptocurrency instrument.
pub struct Instrument {
    name: String,
}

/// A cryptocurrency exchange.
enum Exchange {
    GLOBAL
}

impl PortfolioManager {
    // Example function to be triggered by action (or manually).
    pub fn buy(portfolio: Portfolio, instrument: Instrument, amount: PositionSize) {
        todo!()
    }

    // Example function to be triggered by action (or manually).
    pub fn sell(portfolio: Portfolio, instrument: Instrument, amount: PositionSize) {
        todo!()
    }
}

/// Calculate global PNL.
impl CalculablePNL for PortfolioManager {
    fn pnl(from: DateTime, to: DateTime) -> MoneyType {
        todo!()
    }
}

/// Calculate global total value.
impl CalculableTotalValue for PortfolioManager {
    fn total_value() -> MoneyType {
        todo!()
    }
}

/// Calculate total portfolio PNL.
impl CalculablePNL for Portfolio {
    fn pnl(from: DateTime, to: DateTime) -> MoneyType {
        todo!()
    }
}
