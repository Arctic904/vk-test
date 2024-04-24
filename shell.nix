{ pkgs ? (import <nixpkgs> {}) }:
with pkgs;
mkShell {
  buildInputs = [
    # put packages here.
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    vulkan-tools
    #   vulkan-loader
    #   vulkan-headers
    #   vulkan-cts
    #   vulkan-validation-layers
    #   vulkan-utility-libraries
    # glm and whatnot …
  ];

  # If it doesn’t get picked up through nix magic
  VULKAN_SDK = "${vulkan-validation-layers}/share/vulkan/explicit_layer.d";
}