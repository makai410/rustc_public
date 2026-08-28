{
  description = "Development environment for project-stable-mir josh synchronization";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      runtimeLibs = with pkgs;[
        openssl
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        # Tools executed at build/compile time
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustup
          git
        ];

        # Libraries linked against the binaries
        buildInputs = runtimeLibs;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;

        shellHook = ''
          export CARGO_INSTALL_ROOT=$PWD/.nix-cargo
          export PATH=$CARGO_INSTALL_ROOT/bin:$PATH
        '';
      };
    };
}