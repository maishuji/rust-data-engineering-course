use aws_sdk_s3 as s3;
use aws_config::{BehaviorVersion, Region};

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {

    // Load configuration from the shared credentials file ~/.aws/credentials
    // In my case, my bucket is located at eu-norht-1
    // Be sure to replace "eu-north-1" with your region
    // Be sure to replace "mofafendata" with your bucket
    // Be sure to have set proper policies for your user (ListBuckets and ListObjects)
    let config = aws_config::defaults(BehaviorVersion::latest()).region(Region::new("eu-north-1")).load().await;

    let client = aws_sdk_s3::Client::new(&config);

    // ... make some calls with the client
    //let res = client.list_buckets().send().await?;
    let res = client.list_objects_v2().bucket("mofafendata").send().await?;
    println!("{:?}", res);
    Ok(())
}