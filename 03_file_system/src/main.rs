#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::{data_types::Align, proto::media::file::Directory};
extern crate alloc;

use uefi::proto::media::file::{File, FileInfo};

fn list_dir(volume: &mut Directory, buffer: &mut [u8]) {
    // boucle sur les répertoires
    loop {
        let entry = volume.read_entry(buffer);
        match entry {
            Ok(fileInfoOption) => {
                match fileInfoOption {
                    Some(fileInfo) => {
                        info!(
                            "- filename: {}  size:{}",
                            fileInfo.file_name(),
                            fileInfo.file_size()
                        );
                    },
                    _ => break,
                }

            },
            _ => break,
        }
    }
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    unsafe {
        uefi::allocator::init(&mut system_table);
    }

    // Efface l'écran
    system_table.stdout().clear();

    // Récupère le protocole simple file system à partir de notre image handle
    let mut sfs = system_table
        .boot_services()
        .get_image_file_system(_image_handle)
        .unwrap();

    // Ouvre le volume du disque pour accèder au système de fichier
    let mut volume: uefi::proto::media::file::Directory = sfs.open_volume().unwrap();
    let buffer_size = <FileInfo as Align>::alignment();
    let mut buffer = alloc::vec::Vec::with_capacity(buffer_size);
    buffer.resize(buffer_size * 512, 0);

    list_dir(&mut volume, &mut buffer);
    volume.reset_entry_readout();
    list_dir(&mut volume, &mut buffer);
    volume.close();

    // Pause durant 10 secondes
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
