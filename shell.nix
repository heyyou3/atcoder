# https://nixos.org/guides/nix-pills/developing-with-nix-shell.html
# https://search.nixos.org/packages
with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "atcoder";
    buildInputs = [
        nodejs
        python39
        python39Packages.pip
    ];

    src = ./.;

    buildPhase = ''
    '';

    installPhase = ''
    '';

    shellHook = ''
    export PATH=$PATH:$HOME/.npm-global/bin
    export GOPATH=$HOME/go
    '';
}
