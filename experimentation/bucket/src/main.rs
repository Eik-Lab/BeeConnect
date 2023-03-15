use s3::creds::Credentials;
use s3::error::S3Error;
use s3::Bucket;
use s3::Region;

#[tokio::main]
async fn main() -> Result<(), S3Error> {
    // This requires a running minio server at localhost:9000
    let bucket = Bucket::new(
        "beeapi",
        Region::Custom {
            region: "".to_owned(),
            endpoint: "http://100.124.16.126:9000".to_owned(),
        },
        Credentials {
            access_key: Some("minioadmin".to_string()),
            secret_key: Some("minioadmin".to_string()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )?
    .with_path_style();

    let s3_path = "test1.file";
    // let test = b"I'm going to S3!";
    let test = b"10";

    let response_data = bucket.put_object(s3_path, test).await?;
    assert_eq!(response_data.status_code(), 200);

    let response_data = bucket.get_object(s3_path).await?;
    assert_eq!(response_data.status_code(), 200);
    Ok(())
}
