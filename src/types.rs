/// Stub datetime type.
pub struct DateTime;

/// Use a decimal type in production to avoid precision-related issues.
pub type MoneyType = i64;

pub type InstrumentSize = u64;

/// Implement this to be able to calculate accumulated PNL for an object over a time period.
pub trait CalculablePNL {
    fn pnl(from: DateTime, to: DateTime) -> MoneyType;
}

/// Implement this to be able to calculate total value for an object.
pub trait CalculableTotalValue {
    fn total_value() -> MoneyType;
}
