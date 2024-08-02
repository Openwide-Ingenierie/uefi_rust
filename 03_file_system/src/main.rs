#![no_main]
#![no_std]

use log::info;
use uefi::{prelude::*, CStr16, CStr8, CString16};
use uefi::{data_types::Align, proto::media::file::Directory};
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, RegularFile};
extern crate alloc;

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

fn read_file(filename:&CStr16, volume: &mut Directory, buffer: &mut [u8]) {

    // Ouvre le fichier 
    let readme_txt = volume.open(
        filename,
        FileMode::CreateReadWrite,
        FileAttribute::empty(),
    ).unwrap();

    // Lecture
    unsafe {
        let mut f = RegularFile::new(readme_txt);
        let size = f.read(buffer).unwrap();
        if size < buffer.len() {
            let t = &buffer[0..size as usize];
            info!("{}", CStr8::from_bytes_with_nul_unchecked(t));
        }
        f.close();
    }
}

fn write_file(filename:&CStr16, text: &[u8],  volume: &mut Directory) {

    // Ouvre le fichier 
    let readme_txt = volume.open(
        filename,
        FileMode::CreateReadWrite,
        FileAttribute::empty(),
    ).unwrap();

    // Ecriture
    unsafe {
        let mut f = RegularFile::new(readme_txt);
        let _ = f.write(text);
        let _ = f.flush();
        f.close();
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

    // Ouvre le volume du disque pour accèder au système de fichiers
    let mut volume: uefi::proto::media::file::Directory = sfs.open_volume().unwrap();
    let buffer_size = <FileInfo as Align>::alignment();
    let mut buffer = alloc::vec::Vec::with_capacity(buffer_size);
    buffer.resize(buffer_size * 512, 0);

    list_dir(&mut volume, &mut buffer);
    write_file(cstr16!("readme.txt"), b"hello world !", &mut volume);
    read_file(cstr16!("readme.txt"), &mut volume, &mut buffer);
    volume.reset_entry_readout();
    list_dir(&mut volume, &mut buffer);
    volume.close();

    // Pause durant 10 secondes
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
