// use aws_config::{meta::region::RegionProviderChain, Region};
// use aws_config::BehaviorVersion;
// use aws_sdk_s3::{Client, Error};

use std::str::FromStr;
use tokio;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region = Region::from_str("us-east-1")?;
    //let credentials = Credentials::new(Some("access_key"), Some("secret_key"), None, None, None)?;
    let credentials = Credentials::new(None, None, None, None, None)?;
    let bucket = Bucket::new("asimbucket1", region, credentials)?;

    let results = bucket.list("".to_string(), None).await?;

    for result in results {
        for object in result.contents {
            println!("{}", object.key);
        }
    }

    Ok(())
}




// /// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
//     //let _region:Region = Region::new("us-east-1");

//     let config = aws_config::defaults(BehaviorVersion::latest())
//         .region(region_provider)
//         .load()
//         .await;
//     let client = Client::new(&config);

//     //show_buckets(false, &client, "us-east-1").await?;
//     upload_file(&client).await?;

//     Ok(())
// }

// async fn show_buckets(strict: bool, client: &Client, region: &str) -> Result<(), Error> {
//     let resp = client.list_buckets().send().await?;
//     let buckets = resp.buckets();
//     let num_buckets = buckets.len();

//     let mut in_region = 0;

//     for bucket in buckets {
//         if strict {
//             let r = client
//                 .get_bucket_location()
//                 .bucket(bucket.name().unwrap_or_default())
//                 .send()
//                 .await?;

//             if r.location_constraint().unwrap().as_ref() == region {
//                 println!("{}", bucket.name().unwrap_or_default());
//                 in_region += 1;
//             }
//         } else {
//             println!("{}", bucket.name().unwrap_or_default());
//         }
//     }

//     println!();
//     if strict {
//         println!(
//             "Found {} buckets in the {} region out of a total of {} buckets.",
//             in_region, region, num_buckets
//         );
//     } else {
//         println!("Found {} buckets in all regions.", num_buckets);
//     }

//     Ok(())
// }

// async fn upload_file(client:&Client) -> Result<(), std::io::Error> {
//     //let shared_config = aws_config::load_from_env().await;
//     //let client = S3Client::new(&shared_config);

//     let bucket_name = "asimbucket1".to_string(); // Use your existing bucket
//     let region_provider = RegionProviderChain::first_try(Region::new("us-east-1")); // Use the "us-east-1" region
//     let region = region_provider.region().await.unwrap();

//     let key = "D:\\shoaib_file_aws.txt".to_string(); // Replace with the path to your file

//     // Read the file
//     let data = tokio::fs::read(&key).await?;

//     // Upload the file
//     let _resp = client.put_object()
//         .bucket(&bucket_name)
//         .key(&key)
//         .body(data.into())
//         .send()
//         .await
//         .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

//     println!("File uploaded successfully");

//     Ok(())

// }