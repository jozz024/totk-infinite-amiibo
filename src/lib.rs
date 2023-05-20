use rand::Rng;

extern "C" {
    #[link_name = "_ZN2nn3nfp10GetTagInfoEPNS0_7TagInfoERKNS0_12DeviceHandleE"]
    fn nfpGetTagInfo(taginfo: *mut TagInfo, devicehandle: *mut u8) -> u8;
}

#[repr(C)]
#[derive(Debug, Clone, Copy )]

// it starts with the uid, idk about the rest of it lol
struct TagInfo {
    uid: [u8; 7],
    unk: [u8; 81],
}

#[skyline::hook(replace = nfpGetTagInfo)]
fn nfp_get_tag_info_hook(tag_info: &mut TagInfo, device_handle: *const u8) -> u8 {

    call_original!(tag_info, device_handle);
    // Intercept the current uid and randomly generate a new one
    for i in 1..6 {
        let mut rng = rand::thread_rng();
        tag_info.uid[i] = rng.gen::<u8>();
    }

    return 0
}

#[skyline::main(name = "totk-amiibo-randomizer")]
pub fn main() {
    // panic hook
    // ty to Raytwo for this code
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!("Amiibo Randomizer has panicked at '{}', {}", msg, location);
        skyline::error::show_error(
            69,
            "Amiibo Randomizer has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    skyline::install_hook!(nfp_get_tag_info_hook);
}
