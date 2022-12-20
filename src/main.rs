use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use tokio::io::Error;
use tokio::spawn;

async fn get_file_content(f_name: &str) -> Result<String, Error> {
    let mut buff = String::new();
    let mut f_handel = File::open(f_name).await?;
    f_handel.read_to_string(&mut buff).await?;
    Ok(buff)
}

#[tokio::main]
async fn main() {
    let mut j_vec = vec![];
    let n_vec = vec!["a.txt", "b.txt"];
    for n in n_vec {
        j_vec.push(spawn(get_file_content(n)));
    }
    for j in j_vec {
        println!("Content: {}", j.await.unwrap().unwrap());
    }
}
