use std::time::{Duration, Instant};

use subxt::ext::futures::StreamExt;
use zombienet_sdk_tests::{small_network,  environment::get_spawn_fn};

#[tokio::test(flavor = "multi_thread")]
async fn smoke() {
    tracing_subscriber::fmt::init();

    // config and env
    let spawn_fn = get_spawn_fn();
    let config = small_network().unwrap();

    // spawn
    let now = Instant::now();
    let network = spawn_fn(config).await.unwrap();
    let elapsed = now.elapsed();
    println!("🚀🚀🚀🚀 network deployed in {:.2?}", elapsed);

    // wait a couple of seconds
    tokio::time::sleep(Duration::from_secs(60)).await;
    let alice = network.get_node("alice").unwrap();
    let client = alice.client::<subxt::PolkadotConfig>().await.unwrap();

    // wait 3 blocks
    let mut blocks = client.blocks().subscribe_finalized().await.unwrap().take(10);

    while let Some(block) = blocks.next().await {
        println!("Block #{}", block.unwrap().header().number);
    }

}
