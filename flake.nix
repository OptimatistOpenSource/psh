{
  inputs = {
    pkgs.url = "github:NixOS/nixpkgs/f4341811740ba37cc17962dd1da929bd32dbeb91"; # 24-8-7
    rust-overlay = {
      url = "github:oxalica/rust-overlay/57a1564c924ee4acbffe0ad3d65c7e90d3e77cd8"; # 24-9-8
      inputs.nixpkgs.follows = "pkgs";
    };
    flake-utils.url = "github:numtide/flake-utils/b1d9ab70662946ef0850d488da1c9019f3a9752a"; # 24-3-11
  };

  outputs = inputs@{ ... }: inputs.flake-utils.lib.eachDefaultSystem (system:
    let
      name = "psh";

      pkgs = import inputs.pkgs {
        overlays = [ (import inputs.rust-overlay) ];
        inherit system;
      };
      toolchain = pkgs.rust-bin.stable."1.81.0".complete.override {
        extensions = [ "rust-src" ];
        targets = [ "x86_64-unknown-linux-gnu" ];
      };

      compileTimeDeps = with pkgs; [
        git
        protobuf
        toolchain
        pkg-config
      ];
      runTimeDeps = with pkgs; [
        openssl_3_3.dev
      ];
    in
    {
      devShells.default = pkgs.mkShell {
        inherit name;

        nativeBuildInputs = compileTimeDeps;
        buildInputs = runTimeDeps;
      };
    });
}
