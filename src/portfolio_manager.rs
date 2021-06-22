use crate::types::{CalculablePNL, CalculableTotalValue, DateTime, MoneyType, InstrumentSize};
use crate::static_data_manager::{Instrument, ExchangeType};

/// Manages portfolios for the entire trading system.
pub struct PortfolioManager {
    active_portfolios: Vec<Portfolio>,
}

/// A portfolio for a given exchange (can be global)
pub struct Portfolio {
    exchange: ExchangeType,
    positions: Vec<Position>,
}

/// A cryptocurrency position.
pub struct Position {
    instrument: Instrument,
    size: InstrumentSize,
    transactions: Vec<Transaction>
}

/// A transaction.
pub struct Transaction {
    transaction_type: TransactionType,
    instrument: Instrument,
    size: InstrumentSize
}

/// Whether a transaction is a buy or sell transaction.
pub enum TransactionType {
    BUY,
    SELL
}

impl PortfolioManager {
    /// Example function to be triggered by action (or manually).
    pub fn buy(_portfolio: Portfolio, _instrument: Instrument, _amount: InstrumentSize) {
        todo!()
    }

    /// Example function to be triggered by action (or manually).
    pub fn sell(_portfolio: Portfolio, _instrument: Instrument, _amount: InstrumentSize) {
        todo!()
    }
}

/// Calculate global PNL.
impl CalculablePNL for PortfolioManager {
    fn pnl(_from: DateTime, _to: DateTime) -> MoneyType {
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
    fn pnl(_from: DateTime, _to: DateTime) -> MoneyType {
        todo!()
    }
}

impl Position {

    /// Calculate current size of position.
    fn current_size() -> InstrumentSize {
        todo!()
    }
}

/// Calculate position PNL.
impl CalculablePNL for Position {
    fn pnl(_from: DateTime, _to: DateTime) -> i64 {
        todo!()
    }
}
