## Usage

Put this into Game Settings > General > Launch Options
`gtnkr launch "%command%"`

Example config for Black Desert Online (SteamAppID: 582660) running on Arch Linux | Wayland (Hyprland):

Launch options: `gtnkr launch "%command% --use-d3d11"`

`~/.config/gtnkr/game_configs/582660.ron`
```ron
(
    // https://github.com/FeralInteractive/gamemode is required
    gamemode: true,

    // If you're using gamescope, set mangoapp to true instead of this. mangoapp doesn't seem to work on my system, so I'll keep it.
    // https://github.com/flightlessmango/MangoHud is required
    mangohud: true,

    // There's also Amdvlk but it doesn't work with Black Desert Online
    // https://github.com/FeralInteractive/gamemode is required
    vulkan_driver: Radv,
    gamescope: (
        // There's also Native, it requires https://gitlab.freedesktop.org/xorg/app/xrandr.
        // It also might not work as intended if you've multiple monitors, maybe give it a shot.
        source_resolution: Custom(1920, 1080),

        start_as_fullscreen: true,
        force_grab_cursor: true,
        tearing: true,
        mangoapp: true,

        // There's also Auto, maybe you should use that one.
        backend: Wayland,

        // Should be true if the backend is Wayland, strangely this causes an error on my system.
        expose_wayland: false,
    ),

    environment_variables: [
        ("MESA_VK_WSI_PRESENT_MODE", "immediate"),
		("DXVK_CONFIG", "dxgi.syncInterval=0"),
		("PROTON_ENABLE_NGX_UPDATER", "1")
	]
)
```
