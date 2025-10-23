use std::{
    fs,   //
    io,   //
    path, //
};

/// Returns a list of posts by their path
pub fn get_posts(path_to_posts: &str) -> io::Result<Vec<String>> {
    let posts: Vec<String> = fs::read_dir(path_to_posts)?
        .flat_map(|entry| entry)
        .map(|post| post.path().to_string_lossy().into_owned())
        .collect();

    Ok(posts)
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=posts/");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:warning=Build script is running!");

    let posts_dir = "posts";
    let trunk_output_dir = "dist";

    let posts = get_posts(&posts_dir)?;
    println!("{:?}", &posts);

    let _ = fs::create_dir(&trunk_output_dir);
    let dest_path = path::Path::new(&trunk_output_dir).join("post_list");
    fs::write(
        &dest_path,
        // posts
        //     .iter()
        //     .map(|post| format!("markdown_view!(\"{}\") ", post))
        //     .collect::<String>(),
        format!(
            "{{ {:?} }}
            ",
            &posts
        ),
    )?;

    // let dest_path = path::Path::new(&trunk_output_dir).join("post_views");
    // fs::write(
    //     &dest_path,
    //     format!(
    //         "{{ {} }}",
    //         posts
    //             .iter()
    //             .map(|post| format!("markdown_view!(file = \"{}\");\n", post))
    //             .collect::<String>()
    //     ),
    // )?;

    Ok(())
}
