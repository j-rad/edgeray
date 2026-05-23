use rustray::api::rustray::app::stats::command::stats_service_client::StatsServiceClient;
use rustray::api::rustray::app::stats::command::QueryStatsRequest;
use tonic::transport::Channel;
use tokio_stream::StreamExt;

pub async fn subscribe_telemetry(pattern: String) -> anyhow::Result<impl tokio_stream::Stream<Item = Result<rustray::api::rustray::app::stats::command::QueryStatsResponse, tonic::Status>>> {
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;
    
    let mut client = StatsServiceClient::new(channel);
    let request = QueryStatsRequest {
        pattern,
        reset: false,
    };
    
    let response = client.subscribe_stats(request).await?;
    Ok(response.into_inner())
}
