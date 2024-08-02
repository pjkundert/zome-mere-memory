{
  description = "Holochain Development Env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import ./pkgs.nix {
          pkgs = nixpkgs.legacyPackages.${system};
          inherit system;
        };
	#debugLog = builtins.trace "pkgs output: ${builtins.toJSON pkgs}";
        debugLog = builtins.trace "Available attributes: ${builtins.toString (builtins.attrNames pkgs)}";
      in
      {
        #devShell = {
        #  default = pkgs.mkShell {
        devShell = debugLog (pkgs.mkShell {
            buildInputs = with pkgs; [
              holochain_0-4
              lair-keystore_0-4-5
              hc_0-4

              rustup
              cargo
              rustc

              nodejs_22
            ];

            shellHook = ''
              export PS1="\[\e[1;32m\](flake-env)\[\e[0m\] \[\e[1;34m\]\u@\h:\w\[\e[0m\]$ "
              rustup target add wasm32-unknown-unknown
            '';
        });
        #  };
        #};
      }
    );
}
