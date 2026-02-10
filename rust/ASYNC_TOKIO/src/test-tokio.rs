// CMD Arguments
// ENV Variables
// config file
// stdin
// default
//in CMD EVery thing is string

use clap::{Parser, crate_name, crate_version};
use futures_util::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};







#[derive(Parser, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = "Ali heydari",
    about = "An example app",
    long_about = None
)]
struct Args {
    #[clap(short = 'u', long = "url", help = "The URL to fetch")]
    url: String,
    #[clap(
        short = 'c',
        long = "connections",
        default_value = "8",
        help = "The number of connections to make"
    )]
    connection_count: u8,

    #[clap(short = 'f', long = "file", help = "output file path")]
    out_file_path: Option<String>,
}

async fn partial_d1(url: &str, start: Option<u128>, end: Option<u128>) -> async_tempfile::TempFile {
    let range_string = format!(
        "Ranges={}-{}",
        start.unwrap_or(0),
        end.map_or(String::new(), |x| x.to_string())
    );

    let client = reqwest::Client::new();
    let res = client.get(url).header(reqwest::header::RANGE,range_string).send().await.unwrap();
    let tfile = async_tempfile::TempFile::new().await.unwrap();
    let mut file = tfile.open_rw().await.unwrap();
    let mut stream = res.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        file.write_all(&chunk).await.unwrap();
    }
    tfile
}

#[derive(Debug)]
enum MyError {
    GetfileintoError,
    DoesntSupportResuming,
}

async fn start_download(url: &str, cons: u8,file: &str) -> Result<(), MyError> {
    // HTTP HEAD -> size , accept range
    // range split
    //for each range spawn a task

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap();
    let res = client
        .head(url)
        .send()
        .await
        .map_err(|e| { eprintln!("HEAD request error: {}", e); MyError::GetfileintoError })?;
    eprintln!("HEAD status: {}", res.status());
    eprintln!("HEAD headers: {:#?}", res.headers());
    let accept_range = res
        .headers()
        .get("accept-ranges")
        .ok_or(MyError::DoesntSupportResuming)?;
    println!("{}", accept_range.to_str().unwrap());
    if accept_range != "bytes" {
        return Err(MyError::DoesntSupportResuming);
    }

    let size = res
        .headers()
        .get("content-length")
        .ok_or(MyError::DoesntSupportResuming)?
        .to_str()
        .unwrap()
        .parse::<u128>()
        .unwrap();

    let chunksize = size / cons as u128;

    let u = std::sync::Arc::new(String::from(url));

    let mut tasks = vec![];




    for offset in 0..cons {
        let start = Some((offset + 1) as u128 * chunksize - 1);
        let end = if offset == cons - 1 {
            None
        } else {
            Some(offset as u128 * chunksize)
        };

        let tmp = u.clone();
        tasks.push(tokio::spawn(async move { partial_d1(&tmp, start, end).await },));
    }

    let mut out = tokio::fs::File::create(file).await.unwrap();
    for t in tasks {
        let tf = t.await.unwrap();
        let mut tf2 = tf.open_ro().await.unwrap();
        let mut buf = Vec::with_capacity(4096);

        while tf2.read_buf(&mut buf).await.unwrap() > 0 {
            out.write_all(&buf).await.unwrap();
            buf.clear();
        }

        tokio::fs::remove_file(tf.file_path()).await.unwrap();
    }
    println!("Download completed successfully.");
    Ok(()) // converting Some to Result 
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = args.url;
    let count = args.connection_count;
    let file_path = match args.out_file_path {
        Some(v) => v,
        None => String::from("myfile.bin"),
    };

    match start_download(&url, count,&file_path).await {
        Ok(_) => println!("Hello"),
        Err(MyError::GetfileintoError) => eprintln!("Error: Failed to fetch file info from URL"),
        Err(MyError::DoesntSupportResuming) => {
            eprintln!("Error: Server doesn't support resumable downloads")
        }
    }
}