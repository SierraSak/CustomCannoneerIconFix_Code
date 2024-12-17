#![feature(lazy_cell, ptr_sub_ptr)]
use std::cmp::Ordering;

use engage::gamedata::*;

use unity::prelude::*;

#[unity::class("App", "JobData")]
pub struct JobData2 {
    pub parent: StructBaseFields, //0x0, //0x0
    pub jid: &'static Il2CppString, //0x10
    pub name: &'static Il2CppString, //0x18
    pub aid: &'static Il2CppString, //0x20
    pub help: &'static Il2CppString, //0x28
    pub unit_icon_id_m : Option<&'static Il2CppString>, //0x30
    pub unit_icon_id_f : Option<&'static Il2CppString>, //0x38
    pub unit_icon_weapon_id: &'static Il2CppString, //0x40
    pub rank: i32,  //0x48
    __ : i32,   ///0x4c
    pub style_name: &'static Il2CppString, //0x50
    pub move_type: i32, //0x58
    pub step_frame: i32, //0x5C
    pub max_level: u8, //0x60
    pub internal_level: i8, //0x61
    pub sort: u16, //0x62
    junk: [u8; 0x90], //0x64
    pub skills: Option<&'static mut Il2CppArray<&'static mut Il2CppString>>, //0xf8
    pub learn_skill: Option<&'static Il2CppString>, // 0x100
    pub lunatic_skill: Option<&'static Il2CppString>, //0x108
}

#[unity::hook("App", "JobData", "IsGunner")]
pub fn jobData_IsGunner_hook(this: &mut JobData2, _method_info: OptionalMethod) -> bool {
    if let Some(skills) = this.skills.as_ref(){
        return this.skills.as_ref().unwrap().iter().find(|skill| skill.contains("SID_弾丸装備")).is_some();
    }
    return false;
}

#[skyline::main(name = "CannoneerIconFix")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(
        jobData_IsGunner_hook
    );
}
