{
  description = "✰ArtSCII✰ - ASCII art generator workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        cargoFile = builtins.readFile ./crates/artscii-cli/Cargo.toml;
        cargoToml = cargoFile |> fromTOML;
        appName = (cargoToml.bin |> builtins.head).name;

        artscii = pkgs.rustPlatform.buildRustPackage {
          pname = appName;
          version = cargoToml.package.version;

          src = builtins.path {
            path = ./.;
            name = "artscii-src";
          };
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            libclang
          ];

          buildInputs = with pkgs; [
            ffmpeg
            alsa-lib
          ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = [
            "-isystem" "${pkgs.stdenv.cc.libc_dev}/include"
          ];

          cargoBuildFlags = [
            "-p"
            cargoToml.package.name
          ];

          doCheck = false;

          meta = with pkgs.lib; {
            description = "Convert any image to ASCII art";
            homepage = cargoToml.package.repository;
            license = licenses.mit;
            mainProgram = appName;
          };
        };
      in
      {
        packages.default = artscii;
        packages.artscii = artscii;

        apps.default = flake-utils.lib.mkApp {
          drv = artscii;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            ffmpeg
            pkg-config
            alsa-lib
            libclang
            trunk
            pnpm
            lld
          ];
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        };
      }
    );
}
