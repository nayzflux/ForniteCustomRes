use std::env::home_dir;
use std::fs::File;
use ini::Ini;
use native_windows_gui as nwg;
use native_windows_derive::NwgUi;
use native_windows_gui::NativeUi;

#[derive(Default, NwgUi)]
pub struct FortniteCustomResolution {
    #[nwg_control(size: (300, 150), title: "Fortnite Custom Resolution Utility", position: (300, 300))]
    window: nwg::Window,

    #[nwg_control(text: "Select Screen Resolution:", size: (200, 25), position: (10, 10))]
    resolution_label: nwg::Label,

    #[nwg_control(size: (150, 25), position: (220, 10))]
    resolution_combo: nwg::ComboBox<String>,

    #[nwg_control(text: "Select Custom Format:", size: (200, 25), position: (10, 50))]
    format_label: nwg::Label,

    #[nwg_control(size: (150, 25), position: (220, 50))]
    format_combo: nwg::ComboBox<String>,

    #[nwg_control(text: "Apply", position: (10, 90), size: (150, 30))]
    #[nwg_events( OnButtonClick: [FortniteCustomResolution::apply] )]
    apply_button: nwg::Button,
}

impl FortniteCustomResolution {

    fn apply(&self) {
        let resolution = [[1920, 1080], [2560, 1440]];
        let format = [[16, 9], [4, 3], [5, 4], [21, 9], [32, 9]];

        let f = format[self.format_combo.selection().unwrap()];
        let h = resolution[self.resolution_combo.selection().unwrap()][1];
        let w = h * f[0] / f[1];

        println!("Applying {}x{} ({}:{})", w, h, f[0], f[1]);

        let localappdata = home_dir().unwrap().as_path().to_str().unwrap().to_string();

        let path_string = localappdata + "/AppData/Local/FortniteGame/Saved/Config/WindowsClient/GameUserSettings.ini";
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
        let custom_width = w;
        let custom_height = h;

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

        nwg::modal_info_message(&self.window, "Apply", "Custom resolution has been successfully applied!");
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let app = FortniteCustomResolution::build_ui(Default::default()).expect("Failed to build UI");

    // Ajout des options aux combobox
    app.resolution_combo.insert(0, "1920x1080".to_string());
    app.resolution_combo.insert(1, "2560x1440".parse().unwrap());

    app.format_combo.insert(0, "16:9".parse().unwrap());
    app.format_combo.insert(1, "4:3".parse().unwrap());
    app.format_combo.insert(2, "5:4".parse().unwrap());
    app.format_combo.insert(3, "21:9".parse().unwrap());
    app.format_combo.insert(4, "32:9".parse().unwrap());
    app.format_combo.insert(5, "16:10".parse().unwrap());

    // Lancer la boucle principale de l'interface graphique
    nwg::dispatch_thread_events();
}
