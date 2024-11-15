{
  rustPlatform,
  pkg-config,
  openssl,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "railsy";
  version = "0.1.1";

  src = fetchFromGitHub {
    owner = "mmkaram";
    repo = "railsy";
    rev = "ref/tags/v${version}";
    hash = "";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  buildInputs = [
    openssl
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
}
