#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::ReadDir;
    use std::path::Path;
    use walkdir::WalkDir;

    struct Root {
        document: Option<Document>,
    }

    #[derive(Default, Debug)]
    struct Document {
        pub title: String,
        pub path: String,
        pub menu: Vec<String>,
        pub submenu: Vec<Document>,
    }

    #[test]
    fn path_test() {
        let path = Path::new("/");
        let walker = WalkDir::new(path);
        let iter = walker.into_iter();
        let fil = iter.filter_map(|e| e.ok());

        for entry in fil {
            println!("{:?}", entry);
        }
    }

    pub fn recursion_test(entries: ReadDir, root: &str) -> Document {
        let mut root_docs = Document::default();
        for entry in entries.flatten() {
            let mut sub_docs = Document::default();
            if entry.file_type().unwrap().is_dir() {
                if let Ok(ent) = fs::read_dir(entry.path()) {
                    sub_docs = recursion_test(ent, root)
                }
                root_docs.submenu.push(sub_docs)
            }
            if entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap().contains("md")
            {
                root_docs
                    .menu
                    .push(entry.file_name().to_str().unwrap().to_string())
            }
        }

        root_docs
    }

    #[test]
    fn docs_test() {
        let path = "develop-center-md";
        let sub_path = "document";

        let root_path = Path::new(&path).join(&sub_path);

        let mut root = Root { document: None };

        let mut main_docs = Document::default();
        main_docs.title = String::from("문서");

        if let Ok(entries) = fs::read_dir(&root_path) {
            main_docs = recursion_test(entries, path)
        }

        println!("{:?}", main_docs);

        // let walker = WalkDir::new(&root_path);
        //
        // let iterator = walker.into_iter();
        //
        // let filtered_iterator = iterator.filter_map(|e| e.ok());
        //
        // let mut main_docs = Document::default();
        // main_docs.title = String::from("문서");
        //
        // for entry in filtered_iterator {
        //     // println!(
        //     //     "{:?}",
        //     //     &entry.path().to_str().unwrap()[path.chars().count()..]
        //     // )
        //     println!("file type !! : {:?}", entry.file_type());
        //     if entry.file_type().is_dir() {
        //         let mut sub_docs = Document::default();
        //         sub_docs.title = entry.file_name().to_str().unwrap().to_string();
        //         sub_docs.path =
        //             entry.path().to_str().unwrap()[path.chars().count() + 1..].to_string();
        //         println!("file name : {:?}", entry.file_name());
        //         println!(
        //             "file path : {:?}",
        //             entry.path().to_str().unwrap()[path.chars().count() + 1..].to_string()
        //         );
        //         let root_root_path = Path::new(&root_path).join(entry.file_name());
        //         let walker_sec = WalkDir::new(&root_root_path);
        //         let iter_sec = walker_sec.into_iter();
        //         let fileter_iter = iter_sec.filter_map(|e| e.ok());
        //         for entry_sec in fileter_iter {
        //             println!("sub dir :: {:?}", entry_sec)
        //         }
        //
        //         if entry.file_type().is_file() && entry.file_name().to_str().unwrap().contains("md")
        //         {
        //             println!("{:?}", entry)
        //         }
        //     }
    }
}
