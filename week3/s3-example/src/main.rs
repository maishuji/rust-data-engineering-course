use aws_sdk_s3 as s3;
use aws_config::{BehaviorVersion, Region};

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {

    let config = aws_config::defaults(BehaviorVersion::latest()).region(Region::new("us-east-1")).load().await;

    let client = aws_sdk_s3::Client::new(&config);

    // ... make some calls with the client
    let res = client.list_buckets().send().await?;
    println!("{:?}", res);
    Ok(())
}