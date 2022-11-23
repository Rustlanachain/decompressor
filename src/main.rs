use std::fs;
use std::io;



fn main() {
    std::process::exit(real_main())
}


fn real_main() -> i32{

    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2{
        println!("Usage: {}<filename>",args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len(){
        let mut file = archive.by_index(i).unwrap();
        let output = match file.enclosed_name() {
            Some(path)=>path.to_owned(),
            None => continue,
        };
        {
            let comment=file.comment();
            if !comment.is_empty(){
                println!("file {} comment {}",i,comment);
            }
        }
        if *file.name().ends_with('/') {

            println!("file {} extracted to \"{}",i,outpath.display());
        }

    }
    0
}