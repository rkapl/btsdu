# Btrfs Snapshot Disk Usage Analyzer (btsdu)

This tool can show you what folders have the most changed data between
snapshots. Simply run:

    btsdu -p /snapshots/old_snapshot /snapshots/new_snapshot

You will get a navigable breakdown in a [NCDu](https://dev.yorhel.nl/ncdu)
interface.

```
ncdu 1.13 ~ Use the arrow keys to navigate, press ? for help
--- /home/roman ---------------------------------------------------
                         /..
   42.5 MiB [##########] /.config
   23.2 MiB [#####     ] /.IdeaIC2017.2
   12.3 MiB [##        ] /.vscode
   10.0 MiB [##        ] /projects
    9.5 MiB [##        ] /.thunderbird
  297.3 KiB [          ] /.local
   20.0 KiB [          ] /.java
e   8.0 KiB [          ] /.bash_history
    4.0 KiB [          ] /.pulse
e 293.0   B [          ] /.bashrc
```

# Installation
Use the Rust package manager, `cargo`:

    cargo install btsdu

If the cargo `bin` directory is in your path, you can run `btsdu right away. You
will also need NCDu (should be packaged by your distribution) for proper display
of the results.

# Limitations

 - Shows uncompressed size
 - Does not take into account reflinked files/extents and hardlinks
 - Does show disk usage, just the actual data (called apparent size in NCDu).
   The on-disk and apparent sizes shown will be equal.

# Technical

The tools uses `btrfs send` to get difference between two snapshots and then
builds up file-by-file summary of the size of changed data. The output is
printed in format understood by the NCDu display tool.

Naturally it is possible to use a a btrfs send/receive stream dump from your own
source, see `--help` for that. It is also possible not to run NCDu and just show
the underlying data. It is in pretty reasonable JSON format that can be
processed further.
