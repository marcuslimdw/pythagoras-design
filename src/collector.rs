use async_trait::async_trait;

/// The implementation of the data collector.
struct Collector;

/// An interface for a data source which the collector will connect to and obtain data from.
#[async_trait]
trait DataSource {
    // Connect to a data source.
    async fn connect(uri: &str);

    // Stream data from the data source. Should take care of deserialisation.
    async fn listen();
}
