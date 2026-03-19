{
  description = "Development environment for project-stable-mir josh synchronization";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Define the runtime libraries we need
      runtimeLibs = with pkgs;[
        openssl
        # Add any other dynamic C-libraries here if josh-sync complains (e.g., zlib)
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

        # 🚀 THE FIX: Dynamically populate LD_LIBRARY_PATH for the shell
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;

        shellHook = ''
          export CARGO_INSTALL_ROOT=$PWD/.nix-cargo
          export PATH=$CARGO_INSTALL_ROOT/bin:$PATH
          
          echo "🔧 NixOS Josh-Sync Environment Loaded!"
          echo "LD_LIBRARY_PATH is correctly configured for dynamically linked Cargo binaries."
        '';
      };
    };
}