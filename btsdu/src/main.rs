extern crate clap;
extern crate btrfs_send_parse as bf;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use clap::{App, Arg};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::io;
use std::fs;
use serde::ser::{Serializer, SerializeSeq, Serialize};
use std::process::{exit, Child, Stdio, Command};

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
struct InfoBlockJSON {
    name: String,
    asize: u64,
    dsize: u64,
}

impl Serialize for FileTreeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer
    {
        let mut root = serializer.serialize_seq(Some(1 + self.children.len()))?;
        root.serialize_element(&InfoBlockJSON{name: self.name.clone(), asize: self.changes_size, dsize: self.changes_size})?;
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
    producer: String,
}

#[derive(Serialize)]
struct RootJSON (u32, u32, MetadataJSON, FileTreeNode);

enum InputStream {
    File(Box<dyn io::Read>),
    BtrfsSend(Child)
}
impl InputStream {
    fn get_stream(&mut self) -> &mut io::Read {
        match self {
            InputStream::File(ref mut file) => file,
            InputStream::BtrfsSend(ref mut child) => child.stdout.as_mut().unwrap(),
        }
    }
    fn close(&mut self) {
        if let InputStream::BtrfsSend(child) = self {
            let ret = child.wait().unwrap();
            if !ret.success() {
                panic!("btrfs-send returned {}", ret);
            }
        }
    }
}

fn write_tree(w: &mut io::Write, tree: FileTreeNode) {
    serde_json::ser::to_writer(w, &RootJSON(1, 0, MetadataJSON{producer: "btsdu".to_string()}, tree)).unwrap();
}

fn main() {
    let matches = App::new("btsdu")
        .about("Analyses disc usage of btrfs snapshots in tree forms (using ncdu).")
        .author("Roman Kapl <code@rkapl.cz>")
        .arg(Arg::with_name("input")
            .value_name("INPUT")
            .required(true)
            .help("Subvolume location to analyze. If -s is given, this btrfs-send output msut be specified instead."))
        .arg(Arg::with_name("parent")
            .short("-p")
            .value_name("PARENT")
            .takes_value(true)
            .help("Parent subvolume for incremental snapshot disk usage."))
        .arg(Arg::with_name("send-stream")
            .short("-s")
            .help("Analyze usage of btrfs-send output, instead of on-disk subvolume. \
                  '-' can be given as input in this mode, to read the stream from standard input"))
        .arg(Arg::with_name("raw")
            .short("-r")
            .help("Do not run ncdu, but output usage data in JSON format for later usage."))
        .get_matches();

    if matches.is_present("send-stream") {
        if matches.is_present("parent") {
            eprintln!("The -p option can not be used together with -s.");
            exit(1);
        }
    }
    let mut map = FileMap::new();
    {
        /* Open the correct stream for input */
        let mut stream_source = if matches.is_present("send-stream") {
            let file_path = matches.value_of("input").unwrap();
            if file_path == "-" {
                InputStream::File(Box::new(io::stdin()))
            } else {
                InputStream::File(Box::new(fs::File::open(file_path).unwrap()))
            }
        } else {
            let mut cmd = Command::new("btrfs");
            cmd.arg("send");
            if let Some(parent) = matches.value_of("parent") {
                cmd.arg("-p").arg(parent);
            }
            cmd.arg(matches.value_of("input").unwrap());
            cmd.stdout(Stdio::piped());
            InputStream::BtrfsSend(cmd.spawn().unwrap())
        };
        {
              
            let mut reader = bf::BtrfsReader::new(stream_source.get_stream()).unwrap();
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
        stream_source.close();
    }
    let mut tree = FileTreeNode::new("/");
    for (p, f) in map.map {
        let parts = Vec::from_iter(p.split('/'));
        tree.add(&parts, &f);
    }

    if matches.is_present("raw") {
        write_tree(&mut io::stdout(), tree)
    } else {
        let mut cmd = Command::new("ncdu");
        cmd.arg("-f").arg("-");
        cmd.stdin(Stdio::piped());
        let mut child = cmd.spawn().expect("Can not execute ncdu -- is it installed and in path?");
        write_tree(child.stdin.as_mut().unwrap(), tree);
        child.wait().unwrap();
    }
}
