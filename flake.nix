{
  description = "castle";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
    let pkgs = nixpkgs.legacyPackages.${system}; in 
    with pkgs; rec {
      packages = flake-utils.lib.flattenTree rec {
        castle-shell = mkShell rec {
          LOCALE_ARCHIVE="${glibcLocales}/lib/locale/locale-archive";
          nativeBuildInputs = [
            makeWrapper
            pkg-config
          ];
          buildInputs = [
            rustc cargo bashInteractive coreutils findutils glibcLocales zstd
          ] ++ lib.optionals stdenv.isLinux [
            alsa-lib
            libxkbcommon
            udev
            vulkan-loader
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Cocoa
            rustPlatform.bindgenHook
          ];
          env = {
            ZSTD_SYS_USE_PKG_CONFIG = true;
          };
          shellHook = '' '';
          #postFixup = lib.optionalString stdenv.isLinux ''
          #  patchelf $out/bin/castle \
          #    --add-rpath ${lib.makeLibraryPath [ vulkan-loader ]}
          #'';
        };
      };
      devShell = packages.castle-shell;
      defaultPackage = packages.castle-shell;
    }
  );
}
