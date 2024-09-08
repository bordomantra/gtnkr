## Installation

Nixos:

Add gtnkr to your flake.nix
```nix
inputs = {
    ...
	gtnkr.url = "/home/bordomantra/projects/gtnkr";
    ...
};
```

And also add it to Steam's extra packages:
```nix
programs.steam.extraPackages = [ 
	inputs.gtnkr.packages.${pkgs.system}.default

	# Necessary for gamescope to work as expected, see: https://github.com/NixOS/nixpkgs/issues/162562#issuecomment-1229444338
	pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libXinerama
    pkgs.xorg.libXScrnSaver
    pkgs.libpng
    pkgs.libpulseaudio
    pkgs.libvorbis
    pkgs.stdenv.cc.cc.lib
    pkgs.libkrb5
    pkgs.keyutils
];
```

## Usage

Firstly, take a look at `gtnkr launch --help`.

Put this into Game Settings > General > Launch Options
`gtnkr launch -s "%command%" --log-output`

Example config for Black Desert Online (SteamAppID: 582660) running on Arch Linux | Wayland (Hyprland):

Launch options: `gtnkr launch -s "%command% --use-d3d11" -l`

`~/.config/gtnkr/game_configs/582660.ron`
```ron
(
    gamemode: true,

    // If you're using gamescope, set mangoapp to true instead of this. mangoapp doesn't seem to work on my system, so I'll keep it.
    mangohud: true,

    // There's also Amdvlk but it doesn't work with Black Desert Online
    vulkan_driver: Radv,

    fps_limit: 90,
    gamescope: Some((
		// There's also Custom(width, height) which will work without Hyprland
        source_resolution: Native,

        start_as_fullscreen: true,
        force_grab_cursor: true,
        tearing: true,
        mangoapp: true,

        // There's also Auto, maybe you should use that one.
        backend: Wayland,

        // Should be true if the backend is Wayland, strangely this causes an error on my system.
        expose_wayland: false,
    )),

    environment_variables: [
        ("MESA_VK_WSI_PRESENT_MODE", "immediate"),
		("DXVK_CONFIG", "dxgi.syncInterval=0"),
		("PROTON_ENABLE_NGX_UPDATER", "1")
	]
)
```
