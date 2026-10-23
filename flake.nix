{
  description = "KOMO / mochigotchi — Ratatui TUI (dev shell)";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
          rustfmt
          pkg-config
          openssl
        ];

        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

        shellHook = ''
          echo "KOMO — $(rustc --version)"
        '';
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "mochigotchi";
        version = "1.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        meta.mainProgram = "mochigotchi";
      };
    };
}
