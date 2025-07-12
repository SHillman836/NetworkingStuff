mod handler;

use handler::spawn_connection_tasks;


#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    // spin up 10 independent TCP connections
    spawn_connection_tasks(10).await;
    Ok(())
}