struct StaticDataManager {}

impl StaticDataManager {
    /// Search for an instrument, given some unspecified criteria.
    fn search_instrument() -> Instrument {
        todo!()
    }

    /// Search for an exchange, given some unspecified criteria.
    fn search_exchange() -> Exchange {
        todo!()
    }
}


/// Information about a cryptocurrency instrument.
pub struct Instrument {
    name: String,
    // Other fields
}

/// Information about a cryptocurrency exchange.
pub struct Exchange {
    exchange_type: ExchangeType,
    // Other fields
}

/// A cryptocurrency exchange.
pub enum ExchangeType {
    GLOBAL
}

