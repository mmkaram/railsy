{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };

    railsy = pkgs.rustPlatform.buildRustPackage {
      pname = "railsy";
      version = "0.1.0";

      src = ./.; # Use current directory as source

      cargoLock = {
        lockFile = ./Cargo.lock;
        # outputHashes = {
        #   "package-name-0.1.0" = "sha256-...";
        # };
      };

      buildInputs = with pkgs; [
        openssl
      ];

      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    };
  in {
    packages.${system} = {
      default = railsy;
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rust-analyzer
        openssl
        pkg-config
      ];

      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

      # shellHook = ''
      # '';
    };
  };
}
