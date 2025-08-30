{ pkgs, ... }: {
  packages = [
    pkgs.just
    pkgs.buf
    pkgs.protobuf
    pkgs.protoc-gen-prost
    pkgs.protoc-gen-tonic
    pkgs.protoc-gen-prost-crate
  ];
}
