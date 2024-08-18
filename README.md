Example config for Black Desert Online:

``~/.config/gtnkr/game_configs/582660.ron``
```
(
	gamemode: true,
    mangohud: true,
    vulkan_driver: Radv,
    gamescope: (
        source_resolution: Custom(1920, 1080),
        start_as_fullscreen: true,
        force_grab_cursor: false,
        tearing: true,
		steam_overlay_fix: false,
		mangoapp: true,
		backend: Wayland,
		expose_wayland: false,
    ),
    environment_variables: [
        ("MESA_VK_WSI_PRESENT_MODE", "immediate"),
        ("vk_xwayland_wait_ready", "false"),
		("DXVK_CONFIG", "dxgi.syncInterval=0"),
		("PROTON_ENABLE_NGX_UPDATER", "1")
	]
)
```
