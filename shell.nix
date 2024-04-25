let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";

  pkgs = import <nixpkgs> {
    overlays = [ (import rust-overlay) ];
  };

  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
pkgs.mkShell rec{
  packages = [ toolchain ];

  buildInputs = with pkgs; [
    rustup

    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libxkbcommon

    shaderc
    directx-shader-compiler
    libGL
    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-tools-lunarg
    vulkan-validation-layers
  ];

  # If it doesn’t get picked up through nix magic
  # VULKAN_SDK = "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}"
    export VULKAN_SDK="${pkgs.vulkan-headers}:${pkgs.vulkan-loader}"
    export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"
    export RUST_LOG=info
    export VK_LOADER_DEBUG=none
  '';
}