{ pkgs, ... }: {
  languages.rust.enable = true;

  packages = [
    # Run tasks
    pkgs.just

    # Modify Cargo.toml
    pkgs.cargo-edit

    # Protobuf dependencies
    pkgs.buf
    pkgs.protobuf
    pkgs.protoc-gen-prost
    pkgs.protoc-gen-tonic
    pkgs.protoc-gen-prost-crate
    pkgs.protoc-gen-prost-serde
  ];
}
