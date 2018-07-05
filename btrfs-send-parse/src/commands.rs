use ::*;
#[derive(Clone, Debug)]
pub struct Unknown {
    pub header: CommandHeader,
    pub data: TLVData
}

#[derive(Clone, Debug, Default)]
pub struct Subvol {
    pub path: BtrfsString,
    pub uuid: Uuid,
    pub ctransid: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub path: BtrfsString,
    pub uuid: Uuid,
    pub ctransid: u64,
    pub clone_uuid: Uuid,
    pub clone_ctransid: u64,
}

#[derive(Clone, Debug, Default)]
pub struct MkFile {
    pub path: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct MkDir {
    pub path: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct MkNod {
    pub path: BtrfsString,
    pub mode: u64,
    pub rdev: u64,
}

#[derive(Clone, Debug, Default)]
pub struct MkFifo {
    pub path: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct MkSock {
    pub path: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct SymLink {
    pub path: BtrfsString,
    pub path_link: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct Rename {
    pub path: BtrfsString,
    pub path_to: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct Link {
    pub path: BtrfsString,
    pub path_link: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct UnLink {
    pub path: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct RmDir {
    pub path: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct Write {
    pub path: BtrfsString,
    pub file_offset: u64,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct Clone {
    pub path: BtrfsString,
    pub file_offset: u64,
    pub clone_len: u64,
    pub clone_uuid: Uuid,
    pub clone_ctransid: u64,
    pub clone_path: BtrfsString,
    pub clone_offset: u64,
}

#[derive(Clone, Debug, Default)]
pub struct SetXattr {
    pub path: BtrfsString,
    pub xattr_name: BtrfsString,
    pub xattr_data: Vec<u8>
}

#[derive(Clone, Debug, Default)]
pub struct RemoveXattr {
    pub path: BtrfsString,
    pub xattr_name: BtrfsString,
}

#[derive(Clone, Debug, Default)]
pub struct Truncate {
    pub path: BtrfsString,
    pub size: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Chmod {
    pub path: BtrfsString,
    pub mode: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Chown {
    pub path: BtrfsString,
    pub uid: u64,
    pub gid: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Utimes {
    pub path: BtrfsString,
    pub atime: Timespec,
    pub mtime: Timespec,
    pub ctime: Timespec,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateExtent {
    pub path: BtrfsString,
    pub file_offset: u64,
    pub size: u64,
}

#[derive(Clone, Debug,Default)]
pub struct End {
}