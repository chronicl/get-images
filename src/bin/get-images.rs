use error_chain::error_chain;
use std::{
    fs,
    io::{self, BufRead},
};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

async fn download_image(url: &str, url_number: usize, image_number: &mut u32) -> Result<()> {
    println!("testing url number {} for image", url_number);

    let res = reqwest::get(url).await?;

    // testing if image else return; and getting image type
    let image_type: &str;
    match res.headers().get("content-type") {
        None => {
            println!("No content-type for: {}", url);
            return Ok(());
        }

        Some(content_type_header) => {
            let content_type = content_type_header.to_str().unwrap_or_else(|_err| "");

            if content_type.starts_with("image") {
                let (.., image_type_) = content_type.split_at(6);
                image_type = image_type_;
            } else {
                return Ok(());
            }
        }
    }

    // writing to file
    fs::write(
        format!("pics/{}.{}", image_number, image_type),
        res.bytes().await?,
    )
    .unwrap();
    println!("downloaded image number {}", image_number);
    *image_number += 1;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut file_name = String::new();

    println!("Which file should the urls be read from?: ");
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input.");

    let file = fs::File::open(file_name.trim())?;
    let lines = io::BufReader::new(file).lines();

    let mut image_number: u32 = 1;

    fs::create_dir("pics").except("Couldn't create pics folder");
    for (i, line) in lines.enumerate() {
        download_image(
            &(line.unwrap_or_else(|err| {
                println!("{}", err);
                String::from("")
            })),
            i + 1,
            &mut image_number,
        )
        .await?;
    }
    Ok(())
}
