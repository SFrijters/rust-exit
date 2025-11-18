{
  lib,
  rustPlatform,
  pkg-config,
  dbus,
  gdk-pixbuf,
  graphene,
  gtk4,
}:

rustPlatform.buildRustPackage {
  inherit ((builtins.fromTOML (lib.readFile ./Cargo.toml)).package) name;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    dbus
    gdk-pixbuf
    graphene
    gtk4
  ];

  doCheck = false;

  meta.license = lib.licenses.mit;
}
