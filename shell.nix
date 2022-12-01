let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths =
      [ pkgs.cargo pkgs.clippy pkgs.rust-analyzer pkgs.rustc pkgs.rustfmt ];
  };
in with pkgs;
pkgs.mkShell {

  buildInputs = [ cargo-watch niv nixfmt rust-toolchain ];
}
