use std::fs;
use std::fs::File;
use std::io::prelude::*;
// use std::vec;

use serde::{Serialize};
// use serde_json::Result;

#[derive(Serialize)]
struct ImgFile {
    name : String,
}


/*
purpose: generates a basic static img.json asset for the nuxt framework

eventually this will do more elaborate things.

*/

fn main() -> std::io::Result<()> {

    let mut imgs: Vec<ImgFile> = Vec::new();

    for file in fs::read_dir("./img").unwrap() {
        let fpath = file.unwrap().path();

        dbg!("{}", fpath.display());
        imgs.push(ImgFile{name: fpath.display().to_string()});
    }

    let j = serde_json::to_string(&imgs)?;
    dbg!("{}", &j);

    let mut file = File::create("img.json")?;
    file.write_fmt(format_args!("{}",j))?;

    Ok(())
}
