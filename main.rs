use error_chain::error_chain;
// use std::env::temp_dir;
use std::io::copy;
use std::fs::File;
// use tempfile::Builder;
use std::io::Cursor;
use std::path::PathBuf;


error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let tmp_dir = PathBuf::new();
    let target = "https://wallpapers.com/images/featured-full/samurai-zfa14sdzjl27obzb.jpg";
    let response = reqwest::get(target).await?;
    let mut dest = {
        let fname = response
        .url()
        .path_segments()
        .and_then(|segments | segments.last())
        .and_then(|name | if name.is_empty(){None} else {Some(name)})
        .unwrap_or("tmp.bin");

    println!(" file to  download : '{:?}' ",fname);
    
    // let fname = tmp_dir.path().join(fname);
    let fname = tmp_dir.join(fname);

    println!("will be located under: '{:?}'", fname);
    File::create(fname)?
    };
    let mut content =  Cursor::new(response.bytes().await?);
    copy(&mut content, &mut dest)?;
    Ok(())
}
