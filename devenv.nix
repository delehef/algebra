{ pkgs, lib, config, inputs, ... }:

{
  cachix.enable = false;
  
  # https://devenv.sh/packages/
  packages = [
    # General dev.
    pkgs.git pkgs.openssl
    # Rust crates build deps
    pkgs.openssl
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    version = "2025-09-20";
  };
}
