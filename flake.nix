{
  inputs = {
    #nixurl = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  # outputs =
  #   { self, nixpkgs }:
  #   let
  #     pkgs = import nixpkgs { system = "x86_64-linux"; };
  #   in
  #   {
  #     devShells.x86_64-linux.default = mkShell rec {
  #       nativeBuildInputs = [
  #         pkg-config
  #       ];
  #       buildInputs = [
  #         rustc
  #         cargo
  #         mold

  #         udev
  #         alsa-lib
  #         vulkan-loader
  #         #xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
  #         wayland

  #         libxkbcommon

  #         ############
  #         rust-analyzer

  #         # for cargo fuzz
  #         libiconv
  #         openssl
  #         pkg-config
  #       ];
  #       LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  #     };
  #   };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell rec {
            nativeBuildInputs = [ pkg-config ];
            buildInputs = [
              #rust-bin.beta.latest.default
              #rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
              rust-bin.nightly.latest.default
              cargo
              mold

              udev
              alsa-lib
              vulkan-loader
              # xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
              wayland

              libxkbcommon

              ############
              rust-analyzer

              # for cargo fuzz
              libiconv
              openssl
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            # shellHook = ''
            #   alias ls=eza
            #   alias find=fd
            # '';
          };
      }
    );
}

# nix flake show
# nix flake metadata
# nix develop --command zsh
