#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::text::Key;
use uefi::Char16;
// include pour la recherche de protocols
use uefi::table::boot::SearchType;
use uefi::Identify;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();
    /*
    // récupération du protocol output
    // Récupère un handle sur la sortie de l'écran'
    let output_handle = *system_table.boot_services()
        .locate_handle_buffer(SearchType::ByProtocol(&uefi::proto::console::text::Output::GUID)).unwrap()
        .first()
        .expect("Output is missing");

    // Ouvre le protocol de sortie de l'écran
    let mut output_screen_protocol = system_table.boot_services().open_protocol_exclusive::<uefi::proto::console::text::Output>(output_handle).unwrap();
   */ 
   
    // Efface l'écran
    system_table.stdout().clear();

    // Change la couleur du texte en vert avec un fond en noir 
    system_table.stdout().set_color(
    uefi::proto::console::text::Color::Green,
    uefi::proto::console::text::Color::Black
    );
    
    // Affiche message à l'utilisateur
    system_table.stdout().output_string(cstr16!("Appuyer sur la touche espace pour quitter..."));
   
   // saut à ligne en récupérant la position de la ligne courante
    let row_pos = system_table.stdout().cursor_position().1;
    system_table.stdout().set_cursor_position(0, row_pos + 1);
    
    loop {
        // Récupère l'événement clavier
        if let Some(event) = system_table.stdin().wait_for_key_event() {
            // Attend que l'événement clavier a bien eu lieu
            if system_table.boot_services().wait_for_event(&mut [event]).unwrap() == 0 {
                if let Ok(key_option) = system_table.stdin().read_key() {
                    match key_option  {
                        // vérifie le code ascii de la touche espace
                        Some(key) => {
                            if key == Key::Printable(Char16::try_from(0x20).unwrap()) {
                                break;
                            }
                        },
                        _ => ()
                        }
                    }
            }
        }
        // Pause durant 10 ms
        system_table.boot_services().stall(10_000);
    }
    
    // Affiche un message d'arrêt
    system_table.stdout().output_string(cstr16!("Le programme va s'éteindre..."));
    
    // Pause durant 3 secondes
    system_table.boot_services().stall(3_000_000);
    Status::SUCCESS
}


