{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    # rustup
    rustfmt
    cargo
    gcc
  ];

  shellHook = ''
    echo ""
    echo "Packages loaded: gcc, cargo, rustc, rustup, rustfmt"
  '';

}
