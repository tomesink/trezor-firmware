use crate::{
    strutil::hexlify,
    trezorhal::secbool::secbool,
    ui::{
        ui_features::{ModelUI, UIFeaturesBootloader},
        util::{from_c_array, from_c_str},
    },
};

#[no_mangle]
extern "C" fn screen_welcome() {
    ModelUI::screen_welcome();
}

#[no_mangle]
extern "C" fn bld_continue_label(bg_color: cty::uint16_t) {
    ModelUI::bld_continue_label(bg_color.into());
}

#[no_mangle]
extern "C" fn screen_install_success(
    restart_seconds: u8,
    initial_setup: bool,
    complete_draw: bool,
) {
    ModelUI::screen_install_success(restart_seconds, initial_setup, complete_draw);
}

#[no_mangle]
extern "C" fn screen_install_fail() {
    ModelUI::screen_install_fail();
}

#[no_mangle]
extern "C" fn screen_install_confirm(
    vendor_str: *const cty::c_char,
    vendor_str_len: u8,
    version: *const cty::c_char,
    fingerprint: *const cty::uint8_t,
    should_keep_seed: bool,
    is_newvendor: bool,
    version_cmp: cty::c_int,
) -> u32 {
    let text = unwrap!(unsafe { from_c_array(vendor_str, vendor_str_len as usize) });
    let version = unwrap!(unsafe { from_c_str(version) });

    let mut fingerprint_buffer: [u8; 64] = [0; 64];
    let fingerprint_str = unsafe {
        let fingerprint_slice = core::slice::from_raw_parts(fingerprint, 32);
        hexlify(fingerprint_slice, &mut fingerprint_buffer);
        core::str::from_utf8_unchecked(fingerprint_buffer.as_ref())
    };

    ModelUI::screen_install_confirm(
        text,
        version,
        fingerprint_str,
        should_keep_seed,
        is_newvendor,
        version_cmp,
    )
}

#[no_mangle]
extern "C" fn screen_wipe_confirm() -> u32 {
    ModelUI::screen_wipe_confirm()
}

#[no_mangle]
extern "C" fn screen_unlock_bootloader_confirm() -> u32 {
    ModelUI::screen_unlock_bootloader_confirm()
}

#[no_mangle]
extern "C" fn screen_unlock_bootloader_success() {
    ModelUI::screen_unlock_bootloader_success();
}

#[no_mangle]
extern "C" fn screen_menu(firmware_present: secbool) -> u32 {
    ModelUI::screen_menu(firmware_present)
}

#[no_mangle]
extern "C" fn screen_intro(
    bld_version: *const cty::c_char,
    vendor_str: *const cty::c_char,
    vendor_str_len: u8,
    version: *const cty::c_char,
    fw_ok: bool,
) -> u32 {
    let vendor = unwrap!(unsafe { from_c_array(vendor_str, vendor_str_len as usize) });
    let version = unwrap!(unsafe { from_c_str(version) });
    let bld_version = unwrap!(unsafe { from_c_str(bld_version) });

    ModelUI::screen_intro(bld_version, vendor, version, fw_ok)
}

#[no_mangle]
extern "C" fn screen_boot_stage_1(fading: bool) {
    ModelUI::screen_boot_stage_1(fading)
}

#[no_mangle]
extern "C" fn screen_wipe_progress(progress: u16, initialize: bool) {
    ModelUI::screen_wipe_progress(progress, initialize)
}

#[no_mangle]
extern "C" fn screen_install_progress(progress: u16, initialize: bool, initial_setup: bool) {
    ModelUI::screen_install_progress(progress, initialize, initial_setup)
}

#[no_mangle]
extern "C" fn screen_connect(initial_setup: bool) {
    ModelUI::screen_connect(initial_setup)
}

#[no_mangle]
extern "C" fn screen_wipe_success() {
    ModelUI::screen_wipe_success()
}

#[no_mangle]
extern "C" fn screen_wipe_fail() {
    ModelUI::screen_wipe_fail()
}
