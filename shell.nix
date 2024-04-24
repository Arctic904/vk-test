{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell rec{
  buildInputs = with pkgs; [

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
    export VULKAN_SDK="${vulkan-headers}"
    export VK_LAYER_PATH="${vulkan-validation-layers}/share/vulkan/explicit_layer.d"
  '';
}