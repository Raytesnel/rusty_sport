{
  description = "Interlude Dioxus development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy

            dioxus-cli

            pkg-config
            gtk3
            webkitgtk_4_1
            openssl
            xdotool
          ];

          shellHook = ''
            echo "🧘 Interlude dev shell"
            echo "Rust: $(rustc --version)"
            echo "DX: $(dx --version)"
          '';
        };
      });
}
