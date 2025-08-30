{ pkgs, ... }: {
  languages.rust.enable = true;

  packages = [
    pkgs.just
    pkgs.buf
    pkgs.protobuf
    pkgs.protoc-gen-prost
    pkgs.protoc-gen-tonic
    pkgs.protoc-gen-prost-crate
    pkgs.protoc-gen-prost-serde
  ];
}
