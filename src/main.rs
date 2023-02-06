use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_auth::Project;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::upload::UploadObjectRequest;

use std::{fs::File, io::Read};

#[tokio::main]
async fn main() {
    let credentials = CredentialsFile::new_from_file("./tmp/cred.json".to_string())
        .await
        .unwrap();

    let mut file = File::open("./tmp/test.wav").unwrap();
    let mut audio_bytes = Vec::new();
    file.read_to_end(&mut audio_bytes).unwrap();

    let client = Client::new(ClientConfig {
        project: Some(Project::FromFile(Box::new(credentials))),
        ..Default::default()
    })
    .await
    .unwrap();

    let uploaded = client
        .upload_object(
            &UploadObjectRequest {
                bucket: "bucket".to_string(), // bucket名
                name: "test.wav".to_string(), // ファイル名
                ..Default::default()
            },
            &audio_bytes,
            "audio/wav",
            None,
        )
        .await;

    match uploaded {
        Ok(_) => {
            print!("Upload Complected")
        }
        Err(err) => {
            println!("[Error]: failed to Upload Error: {}", err)
        }
    }
}
