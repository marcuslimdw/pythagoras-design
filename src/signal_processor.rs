use async_trait::async_trait;

use crate::portfolio_manager::PortfolioManager;
use crate::signal_builder::Signal;

/// Processes data from the data collector to determine if signals should be sent to the portfolio
/// manager.
struct SignalProcessor {
    portfolio_manager: PortfolioManager,
}

/// Data collector input.
pub struct DataCollectorPoint {
    point_type: DataCollectorPointType,
}

/// Type of data collector input.
enum DataCollectorPointType {}

impl SignalProcessor {
    /// Register a signal to perform certain actions.
    fn register_signal(_signal: Signal) {
        todo!()
    }

    /// Unregister a previously registered signal.
    fn unregister_signal(_signal: Signal) {
        todo!()
    }

    /// Handle input from the data collector, converting it into StorageData and writing it if
    /// necessary.
    async fn handle_input(_input: DataCollectorPoint) {
        todo!()
    }
}

/// Signal processor output, to be written to storage.
struct StorageData;

/// An interface for storage.
#[async_trait]
trait Storage {
    /// Connect to a storage.
    async fn connect(uri: &str);

    /// Check whether certain data should be written to this storage.
    fn check_responsibility(_data: StorageData) -> bool {
        todo!()
    }

    /// Write data to storage.
    async fn write();
}
