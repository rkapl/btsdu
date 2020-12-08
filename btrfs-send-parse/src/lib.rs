extern crate byteorder;
pub mod definitions;
pub mod commands;
use definitions::*;

use std::io;
use std::io::{Read, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};

pub type Error = io::Error;
pub type Result<T> = io::Result<T>;

pub struct BtrfsReader<'a> {
    r: &'a mut Read,
    version: u32,
}

#[derive(Clone, Debug)]
pub enum Command {
    Unknown(commands::Unknown),
    Subvol(commands::Subvol),
    Snapshot(commands::Snapshot),
    MkFile(commands::MkFile),
    MkDir(commands::MkDir),
    MkNod(commands::MkNod),
    MkFifo(commands::MkFifo),
    MkSock(commands::MkSock),
    SymLink(commands::SymLink),
    Rename(commands::Rename),
    Link(commands::Link),
    UnLink(commands::UnLink),
    RmDir(commands::RmDir),
    Write(commands::Write),
    Clone(commands::Clone),
    SetXattr(commands::SetXattr),
    RemoveXattr(commands::RemoveXattr),
    Truncate(commands::Truncate),
    Chmod(commands::Chmod),
    Chown(commands::Chown),
    Utimes(commands::Utimes),
    UpdateExtent(commands::UpdateExtent),
    End(commands::End),
}

#[derive(Clone, Debug)]
pub struct CommandHeader {
    pub len: u32,
    pub cmd: u16,
    pub crc32: u32,
}

#[derive(Clone, Debug)]
pub struct TLVData {
    pub entries: Vec<TLVEntry>,
}

#[derive(Clone, Debug)]
pub struct TLVEntry {
    pub key: u16,
    pub value: Vec<u8>,
}

pub const UUID_SIZE:usize = 16;
#[derive(Clone, Debug, Copy, Default)]
pub struct Uuid {
    pub data: [u8; UUID_SIZE],
}
#[derive(Clone, Debug, Copy, Default)]
pub struct Timespec {
    pub sec: u64,
    pub nsec: u32,
}

pub type BtrfsString = String;

fn invalid_data<T>(err: &str) -> Result<T> {
    panic!("Test");
    return Err(io::Error::new(io::ErrorKind::InvalidData, err));
}

impl From<u16> for Cmd {
	fn from(c: u16) -> Cmd{
		unsafe {
			std::mem::transmute_copy::<u16, Cmd>(&c)
		}
	}
}

impl From<u16> for Attr {
	fn from(c: u16) -> Attr{
		unsafe {
			std::mem::transmute_copy::<u16, Attr>(&c)
		}
	}
}

impl TLVData {
    pub fn get(&self, key: u16) -> Result<&Vec<u8>> {
        for e in self.entries.iter() {
            if e.key == key {
                return Ok(&e.value)
            }
        }
        invalid_data(&format!("item {} does not exist", key))
    }
    pub fn get_u8(&self, key: u16) -> Result<u8> {
        match Cursor::new(try!(self.get(key)).clone()).bytes().next() {
            None => Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
            Some(x) => x,
        }
    }
    pub fn get_u16(&self, key: u16) -> Result<u16> {
        Cursor::new(try!(self.get(key))).read_u16::<LittleEndian>()
    }
    pub fn get_u32(&self, key: u16) -> Result<u32> {
        Cursor::new(try!(self.get(key))).read_u32::<LittleEndian>()
    }
    pub fn get_u64(&self, key: u16) -> Result<u64> {
        Cursor::new(try!(self.get(key))).read_u64::<LittleEndian>()
    }
    pub fn get_string(&self, key: u16) -> Result<BtrfsString> {
        // TODO: make the conversion function configurable
        match String::from_utf8(try!(self.get(key)).clone()) {
            Ok(x) => Ok(x),
            Err(_   ) => invalid_data("utf-8 decoding problem"),
        }
    }
    pub fn get_timespec(&self, key: u16) -> Result<Timespec> {
        let mut r = Cursor::new(try!(self.get(key)));
        Ok(Timespec {
            sec: try!(r.read_u64::<LittleEndian>()),
            nsec: try!(r.read_u32::<LittleEndian>()),
        })
    }
    pub fn get_uuid(&self, key: u16) -> Result<Uuid> {
        let data = try!(self.get(key));
        let mut uuid = Uuid{data: [0u8; UUID_SIZE]};
        try!(Cursor::new(data).read_exact(&mut uuid.data));
        Ok(uuid)
    }
    pub fn uuid(&self) -> Result<Uuid> {
        self.get_uuid(Attr::UUID as u16)
    }
    pub fn clone_uuid(&self) -> Result<Uuid> {
        self.get_uuid(Attr::CLONE_UUID as u16)
    }
    pub fn path(&self) -> Result<BtrfsString> {
        self.get_string(Attr::PATH as u16)
    }
    pub fn clone_path(&self) -> Result<BtrfsString> {
        self.get_string(Attr::CLONE_PATH as u16)
    }
    pub fn path_link(&self) -> Result<BtrfsString> {
        self.get_string(Attr::PATH_LINK as u16)
    }
    pub fn path_to(&self) -> Result<BtrfsString> {
        self.get_string(Attr::PATH_TO as u16)
    }
    pub fn ctransid(&self) -> Result<u64> {
        self.get_u64(Attr::CTRANSID as u16)
    }
    pub fn clone_ctransid(&self) -> Result<u64> {
        self.get_u64(Attr::CLONE_CTRANSID as u16)
    }
    pub fn file_offset(&self) -> Result<u64> {
        self.get_u64(Attr::FILE_OFFSET as u16)
    }
    pub fn clone_offset(&self) -> Result<u64> {
        self.get_u64(Attr::CLONE_OFFSET as u16)
    }
    pub fn clone_len(&self) -> Result<u64> {
        self.get_u64(Attr::CLONE_LEN as u16)
    }
    pub fn mode(&self) -> Result<u64> {
        self.get_u64(Attr::MODE as u16)
    }
    pub fn uid(&self) -> Result<u64> {
        self.get_u64(Attr::UID as u16)
    }
    pub fn gid(&self) -> Result<u64> {
        self.get_u64(Attr::GID as u16)
    }
    pub fn rdev(&self) -> Result<u64> {
        self.get_u64(Attr::RDEV as u16)
    }
    pub fn size(&self) -> Result<u64> {
        self.get_u64(Attr::SIZE as u16)
    }
    pub fn data(&self) -> Result<&Vec<u8>> {
        self.get(Attr::DATA as u16)
    }
    pub fn xattr_data(&self) -> Result<&Vec<u8>> {
        self.get(Attr::XATTR_DATA as u16)
    }
    pub fn xattr_name(&self) -> Result<BtrfsString> {
        self.get_string(Attr::XATTR_NAME as u16)
    }
    pub fn atime(&self) -> Result<Timespec> {
        self.get_timespec(Attr::ATIME as u16)
    }
    pub fn mtime(&self) -> Result<Timespec> {
        self.get_timespec(Attr::MTIME as u16)
    }
    pub fn ctime(&self) -> Result<Timespec> {
        self.get_timespec(Attr::CTIME as u16)
    }
}

/* Returns if eof was encountered on first read (true = eof) */
fn read_exact_with_eof(r: &mut io::Read, buf: &mut [u8]) -> Result<bool>{
    if buf.len() == 0 {
        return Ok(false)
    }
    let size = try!(r.read(buf));
    if size == 0 {
        return Ok(true)
    }
    try!(r.read_exact(&mut buf[size..]));
    Ok(false)
}

impl<'a> BtrfsReader<'a> {
    pub fn new(r: &mut Read) -> Result<BtrfsReader> {
        let mut magic_buf = [0u8; MAGIC_LEN];
        try!(r.read_exact(&mut magic_buf));
        if magic_buf != MAGIC.as_bytes() {
            return invalid_data("btrfs stream header does not match");
        }
        let version = try!(r.read_u32::<LittleEndian>());
        if version != 1 {
            return invalid_data("unsupported btrfs stream version");
        }
        Ok(BtrfsReader{r: r, version: version})
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn read_command(&mut self) -> Result<Option<Command>> {
        let cmd = try!(self.read_generic_command());
        match cmd {
            Some(cmd) => self.parse_command(cmd).map(|x| Some(x)),
            None => Ok(None)
        }
    }
    pub fn parse_command(&self, cmd: commands::Unknown) -> Result<Command> {
        {
            // println!("Parsing {}", cmd.header.cmd);
            let t = &cmd.data;
            match Cmd::from(cmd.header.cmd) {
                Cmd::SUBVOL => return Ok(Command::Subvol(commands::Subvol {
                    path: try!(t.path()),
                    uuid: try!(t.uuid()),
                    ctransid: try!(t.ctransid()),
                })),
                Cmd::SNAPSHOT => return Ok(Command::Snapshot(commands::Snapshot {
                    path: try!(t.path()),
                    uuid: try!(t.uuid()),
                    ctransid: try!(t.ctransid()),
                    clone_ctransid: try!(t.clone_ctransid()),
                    clone_uuid: try!(t.clone_uuid()),
                })),
                Cmd::MKFILE => return Ok(Command::MkFile(commands::MkFile {
                    path: try!(t.path()),
                })),
                Cmd::MKNOD => return Ok(Command::MkNod(commands::MkNod {
                    path: try!(t.path()),
                    mode: try!(t.mode()),
                    rdev: try!(t.rdev()),
                })),
                Cmd::MKFIFO => return Ok(Command::MkFifo(commands::MkFifo {
                    path: try!(t.path())
                })),
                Cmd::MKSOCK => return Ok(Command::MkSock(commands::MkSock {
                    path: try!(t.path())
                })),
                Cmd::SYMLINK => return Ok(Command::SymLink(commands::SymLink {
                    path: try!(t.path()),
                    path_link: try!(t.path_link()),
                })),
                Cmd::RENAME => return Ok(Command::Rename(commands::Rename {
                    path: try!(t.path()),
                    path_to: try!(t.path_to()),
                })),
                Cmd::LINK => return Ok(Command::Link(commands::Link {
                    path: try!(t.path()),
                    path_link: try!(t.path_link()),
                })),
                Cmd::UNLINK => return Ok(Command::UnLink(commands::UnLink {
                    path: try!(t.path()),
                })),
                Cmd::RMDIR => return Ok(Command::RmDir(commands::RmDir {
                    path: try!(t.path()),
                })),
                Cmd::WRITE => return Ok(Command::Write(commands::Write {
                    path: try!(t.path()),
                    file_offset: try!(t.file_offset()),
                    data: try!(t.data()).clone(),
                })),
                Cmd::CLONE => return Ok(Command::Clone(commands::Clone {
                    path: try!(t.path()),
                    file_offset: try!(t.file_offset()),
                    clone_len: try!(t.clone_len()),
                    clone_uuid: try!(t.clone_uuid()),
                    clone_ctransid: try!(t.clone_ctransid()),
                    clone_path: try!(t.clone_path()),
                    clone_offset: try!(t.clone_offset()),
                })),
                Cmd::SET_XATTR => return Ok(Command::SetXattr(commands::SetXattr {
                    path: try!(t.path()),
                    xattr_name: try!(t.xattr_name()),
                    xattr_data: try!(t.xattr_data()).clone(),
                })),
                Cmd::REMOVE_XATTR => return Ok(Command::RemoveXattr(commands::RemoveXattr {
                    path: try!(t.path()),
                    xattr_name: try!(t.xattr_name()),
                })),
                Cmd::TRUNCATE => return Ok(Command::Truncate(commands::Truncate {
                    path: try!(t.path()),
                    size: try!(t.size()),
                })),
                Cmd::CHMOD => return Ok(Command::Chmod(commands::Chmod {
                    path: try!(t.path()),
                    mode: try!(t.mode()),
                    
                })),
                Cmd::CHOWN => return Ok(Command::Chown(commands::Chown {
                    path: try!(t.path()),
                    uid: try!(t.uid()),
                    gid: try!(t.gid()),
                    
                })),
                Cmd::UTIMES => return Ok(Command::Utimes(commands::Utimes {
                    path: try!(t.path()),
                    atime: try!(t.atime()),
                    mtime: try!(t.mtime()),
                    ctime: try!(t.ctime()),
                })),
                Cmd::UPDATE_EXTENT => return Ok(Command::UpdateExtent(commands::UpdateExtent {
                    path: try!(t.path()),
                    file_offset: try!(t.file_offset()),
                    size: try!(t.size()),
                })),
                Cmd::END => return Ok(Command::End(commands::End{})),
                _ => {}
            }
        }
        Ok(Command::Unknown(cmd))
    }
    pub fn read_generic_command(&mut self) -> Result<Option<commands::Unknown>> {
        Ok(match try!(self.read_command_header()) {
            Some(header) => Some(commands::Unknown {
                header: header.clone(),
                data: try!(self.read_tlvs(header.len)),
            }),
            None => None,
        })  
    }
    pub fn read_command_header(&mut self) -> Result<Option<CommandHeader>> {
        let mut len_buf = [0u8; 4];
        let eof = try!(read_exact_with_eof(self.r, &mut len_buf))   ;
        if eof {
            return Ok(None)
        }
        Ok(Some(CommandHeader {
            len: try!(Cursor::new(len_buf).read_u32::<LittleEndian>()),
            cmd: try!(self.r.read_u16::<LittleEndian>()),
            crc32: try!(self.r.read_u32::<LittleEndian>()),
        }))
    }
    pub fn read_tlvs(&mut self, len_to_read: u32) -> Result<TLVData> {
        let mut tlv_data = TLVData {entries:Vec::new()};
        let mut remaining = len_to_read;
        while remaining > 0 {
            let key = try!(self.r.read_u16::<LittleEndian>());
            let len = try!(self.r.read_u16::<LittleEndian>());
            let mut data = vec![0; len as usize];
            try!(self.r.read_exact(data.as_mut_slice()));
            tlv_data.entries.push(TLVEntry{key: key, value: data});
            remaining -= 2*2 + len as u32;
        }
        Ok(tlv_data)
    }
}

pub struct CommandPrintOptions {
}

impl Default for CommandPrintOptions {
    fn default() -> Self {
        CommandPrintOptions {}
    }
}

impl Command {
    pub fn print(&self, f: &mut std::io::Write, _opts: &CommandPrintOptions) -> std::io::Result<()> {
        match self {
            Command::Write(w) => 
                write!(f, "Write {{ path = {:?}, file_offset = {}, data_len = {} }}\n", w.path, w.file_offset, w.data.len()),   
            _ => write!(f, "{:?}\n", self)
        }
    }
}
