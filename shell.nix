{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    rustfmt
    cargo
    gcc
  ];

  shellHook = ''
    echo ""
    echo "Packages loaded: gcc, cargo, rustc, rustfmt"
  '';

}
