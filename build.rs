use std::{fs, io};

pub fn get_posts(path_to_posts: &str) -> io::Result<Vec<String>> {
    let posts: Vec<String> = vec![];
    for entry in fs::read_dir(path_to_posts)? {
        let post = entry?;
        println!("{:?}", post);
    }
    Ok(posts)
}

fn main() -> io::Result<()> {
    let posts = get_posts("posts")?;
    println!("{:?}", posts);
    Ok(())
}
