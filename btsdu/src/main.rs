extern crate clap;
extern crate btrfs_send_parse as bf;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use clap::App;
use std::collections::HashMap;
use std::iter::FromIterator;
use serde::ser::{Serializer, SerializeSeq, Serialize};

struct FileInfo {
    changes_size: u64,
}

struct FileMap {
    map: HashMap<String, FileInfo>,
}

impl FileMap {
    fn new() -> Self {
        FileMap {map: HashMap::new()}
    }
    fn get(&mut self, key: &str) -> &mut FileInfo{
        self.map.entry(key.to_string()).or_insert(FileInfo {changes_size: 0})
    }
    fn acc(&mut self, key: &str, amount: u64) {
        self.get(key).changes_size += amount
    }
    fn rename(&mut self, old: &str, new: &str) {
        if let Some(v) = self.map.remove(old) {
            *self.get(new) = v;
        }
    }
}

struct FileTreeNode {
    name: String,
    children: HashMap<String, Box<FileTreeNode>>,
    changes_size: u64,
}

#[derive(Serialize)]
struct InfoBlock {
    name: String,
    asize: u64,
    dsize: u64,
}

impl Serialize for FileTreeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer
    {
        let mut root = serializer.serialize_seq(Some(1 + self.children.len()))?;
        root.serialize_element(&InfoBlock{name: self.name.clone(), asize: self.changes_size, dsize: self.changes_size})?;
        for c in self.children.values() {
            root.serialize_element(c)?;
        }
        root.end()
    }
}

impl FileTreeNode {
    fn new(name: &str) -> FileTreeNode {
        FileTreeNode {children: HashMap::new(), changes_size: 0, name: name.to_string()}
    }
    fn add(&mut self, path: &[&str], info: &FileInfo) {
        if path.len() == 0 {
            self.changes_size += info.changes_size;
        } else {
            let f = self.children.entry(path[0].to_string())
                .or_insert_with(|| Box::new(FileTreeNode::new(path[0])));
            f.add(&path[1..], info);
        }
    }
}

#[derive(Serialize)]
struct MetadataJSON {

}

#[derive(Serialize)]
struct RootJSON (u32, u32, MetadataJSON, FileTreeNode);

fn main() {
    let matches = App::new("btsdu")
        .about("Analyses disc usage of btrfs snapshots in tree forms (using ncdu).")
        .author("Roman Kapl <code@rkapl.cz>")
        .get_matches();

    let mut map = FileMap::new();
    {
        let mut stream = std::io::stdin();
        let mut reader = bf::BtrfsReader::new(&mut stream).unwrap();
        let mut cmd_count = 0;
        loop {
            match reader.read_command().unwrap() {
                None => break,
                Some(bf::Command::Rename(c)) => map.rename(&c.path, &c.path_to),
                Some(bf::Command::Write(c)) => map.acc(&c.path, c.data.len() as u64),
                Some(bf::Command::SetXattr(c)) => map.acc(&c.path, c.xattr_data.len() as u64),
                _ => {},
            }
            cmd_count += 1;
        }
        eprintln!("Processed {} commands", cmd_count);
    }
    let mut tree = FileTreeNode::new("/");
    for (p, f) in map.map {
        let parts = Vec::from_iter(p.split('/'));
        tree.add(&parts, &f);
    }
    serde_json::ser::to_writer_pretty(std::io::stdout(), &RootJSON(1, 0, MetadataJSON{}, tree)).unwrap();
}
