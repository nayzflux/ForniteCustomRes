use ini::Ini;
use dialoguer::Select;
use std::fs::File;
use std::env::home_dir;

fn main() {
    println!("Welcome to my Fornite Custom Res Utility");

    let localappdata = home_dir().unwrap().as_path().to_str().unwrap().to_string();

    let sizes = vec!["1920x1080", "2560x1440"];

    let res = Select::new()
        .with_prompt("What's your Screen Resolution?")
        .items(&sizes)
        .default(0)
        .interact()
        .unwrap();

    println!("Screen Resolution: {}", sizes[res]);

    let mut height = 0;

    if sizes[res].eq("1920x1080") {
        height = 1080;
    }

    if sizes[res].eq("2560x1440") {
        height = 1440;
    }

    let formats = vec!["16:9", "4:3", "5:4", "21:9", "32:9", "16:10"];

    let format = Select::new()
        .with_prompt("Select a Custom Format?")
        .items(&formats)
        .default(0)
        .interact()
        .unwrap();

    println!("Custom Format: {}", formats[format]);

    let mut num = 0;
    let mut den = 0;

    if formats[format].eq("16:9") {
        num = 16;
        den = 9;
    }

    if formats[format].eq("4:3") {
        num = 4;
        den = 3;
    }

    if formats[format].eq("5:4") {
        num = 5;
        den = 4;
    }

    let path_string = localappdata + "/FortniteGame/Saved/Config/WindowsClient/GameUserSettings.ini";
    let path = path_string.as_str();

    println!("{}", path);

    // Make config not read only
    println!("Loading GameUserSettings.ini...");

    let f = File::open(path).unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();

    println!("Disabling read-only");

    permissions.set_readonly(false);

    /**
        Set custom resolution

        LastUserConfirmedDesiredScreenWidth=1920
        LastUserConfirmedDesiredScreenHeight=1440
        DesiredScreenWidth=1920
        DesiredScreenHeight=1440
        FrameRateLimit=0.000000
        LastConfirmedFullscreenMode=0
        PreferredFullscreenMode=0
        ResolutionSizeX=1920
        ResolutionSizeY=1440
        LastUserConfirmedResolutionSizeX=1920
        LastUserConfirmedResolutionSizeY=1440
    **/

    // Calculate Width
    let custom_width = (height * num) / den;
    let custom_height = height;

    let mut conf = Ini::load_from_file(path).unwrap();

    println!("Upating GameUserSettings.ini...");

    println!("Custom Resolution:");
    println!("{}", custom_width);
    println!("{}", custom_height);

    conf.with_section(Some("/Script/FortniteGame.FortGameUserSettings"))
        // Set resolution size
        .set("LastUserConfirmedResolutionSizeX", custom_width.to_string())
        .set("LastUserConfirmedResolutionSizeY", custom_height.to_string())
        .set("ResolutionSizeX", custom_width.to_string())
        .set("ResolutionSizeY", custom_height.to_string())
        // Set screen size
        .set("LastUserConfirmedDesiredScreenWidth", custom_width.to_string())
        .set("LastUserConfirmedDesiredScreenHeight", custom_height.to_string())
        .set("DesiredScreenWidth", custom_width.to_string())
        .set("DesiredScreenHeight", custom_height.to_string())
        // Set to fullscreen
        .set("PreferredFullscreenMode", "0")
        .set("LastConfirmedFullscreenMode", "0");

    // Save
    conf.write_to_file(path).unwrap();

    println!("GameUserSettings.ini saved");

    permissions.set_readonly(true);
}