use binaryninja::{
    binary_view::{BinaryView, BinaryViewBase},
    command::{RangeCommand, register_command_for_range},
    interaction::{get_save_filename_input, show_message_box},
};

use binaryninjacore_sys::{BNMessageBoxButtonSet, BNMessageBoxIcon};

use std::ops::Range;
use std::{fs::File, io};

const BUF_SIZE: usize = 1024;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn UIPluginInit() -> bool {
    register_command_for_range("Dump to file", "Dumps selection to a file", Dumper::new());

    true
}

trait BlockingRead {
    fn read_blocking(&self, buf: &mut [u8], offset: u64);
}

impl BlockingRead for BinaryView {
    fn read_blocking(&self, buf: &mut [u8], offset: u64) {
        let mut read_bytes = 0;
        while read_bytes < buf.len() {
            read_bytes += self.read(&mut buf[read_bytes..], offset + read_bytes as u64);
        }
    }
}

trait BlockingWrite {
    fn write_blocking(&mut self, buf: &[u8]) -> io::Result<()>;
}

impl<T: std::io::Write> BlockingWrite for T {
    fn write_blocking(&mut self, buf: &[u8]) -> io::Result<()> {
        let mut written_bytes = 0;
        while written_bytes < buf.len() {
            written_bytes += self.write(&buf[written_bytes..])?;
        }

        Ok(())
    }
}

struct Dumper {}

impl Dumper {
    fn new() -> Self {
        Dumper {}
    }

    fn dump_range(&self, view: &BinaryView, range: Range<u64>) {
        let output_file =
            match get_save_filename_input("Specify output file", "Dump data", "dump.bin") {
                Some(p) => p,
                None => {
                    show_message_box(
                        "Error: Dump file",
                        "No File given! Will not dump.",
                        BNMessageBoxButtonSet::OKButtonSet,
                        BNMessageBoxIcon::ErrorIcon,
                    );
                    return;
                }
            };

        let mut fd = match File::create(&output_file) {
            Ok(fd) => fd,
            Err(e) => {
                show_message_box(
                    "Error: Dump file",
                    &format!("Failed to open \"{}\": {:?}", output_file.display(), e),
                    BNMessageBoxButtonSet::OKButtonSet,
                    BNMessageBoxIcon::ErrorIcon,
                );
                return;
            }
        };

        let len = range.end - range.start;
        let mut buf = [0u8; BUF_SIZE];
        let mut written = 0;

        while written != len {
            let tbr = if len - written > BUF_SIZE as u64 {
                BUF_SIZE as u64
            } else {
                len - written
            };

            view.read_blocking(&mut buf[0..tbr as usize], range.start + written);

            if let Err(e) = fd.write_blocking(&buf[0..tbr as usize]) {
                show_message_box(
                    "Error Dump File",
                    &format!("Failed to write to \"{}\": {:?}", output_file.display(), e),
                    BNMessageBoxButtonSet::OKButtonSet,
                    BNMessageBoxIcon::ErrorIcon,
                );
                return;
            }

            written += tbr;
        }
    }

    fn check_range(&self, view: &BinaryView, range: Range<u64>) -> bool {
        view.offset_readable(range.start) && view.offset_readable(range.end - 1)
    }
}

impl RangeCommand for Dumper {
    fn action(&self, view: &BinaryView, range: Range<u64>) {
        self.dump_range(view, range);
    }

    fn valid(&self, view: &BinaryView, range: Range<u64>) -> bool {
        self.check_range(view, range)
    }
}
