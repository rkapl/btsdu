#![allow(non_camel_case_types)]

pub const MAGIC: &str = "btrfs-stream\0";
pub const MAGIC_LEN: usize = 13;

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum Cmd {
	UNSPEC,

	SUBVOL,
	SNAPSHOT,

	MKFILE,
	MKDIR,
	MKNOD,
	MKFIFO,
	MKSOCK,
	SYMLINK,

	RENAME,
	LINK,
	UNLINK,
	RMDIR,

	SET_XATTR,
	REMOVE_XATTR,

	WRITE,
	CLONE,

	TRUNCATE,
	CHMOD,
	CHOWN,
	UTIMES,

	END,
	UPDATE_EXTENT,
	__MAX,
}

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum Attr {
	UNSPEC,

	UUID,
	CTRANSID,

	INO,
	SIZE,
	MODE,
	UID,
	GID,
	RDEV,
	CTIME,
	MTIME,
	ATIME,
	OTIME,

	XATTR_NAME,
	XATTR_DATA,

	PATH,
	PATH_TO,
	PATH_LINK,

	FILE_OFFSET,
	DATA,

	CLONE_UUID,
	CLONE_CTRANSID,
	CLONE_PATH,
	CLONE_OFFSET,
	CLONE_LEN,

	__MAX,
}