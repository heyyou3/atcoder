# https://nixos.org/guides/nix-pills/developing-with-nix-shell.html
# https://search.nixos.org/packages
with import <nixpkgs> {};

let
  my-python-packages = python-packages: with python-packages; [
      online-judge-tools
  ];
  python-with-my-packages = python3.withPackages my-python-packages;
in stdenv.mkDerivation {
    name = "atcoder";
    buildInputs = [
        nodejs
    ];

    src = ./.;

    buildPhase = ''
    '';

    installPhase = ''
    pip install --user online-judge-tools
    npm install -g atcoder-cli
    '';

    shellHook = ''
    export PATH=$PATH:$HOME/.npm-global/bin
    export GOPATH=$HOME/go
    '';
}
