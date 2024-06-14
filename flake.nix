{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in
        with pkgs; {
          formatter = alejandra;
          devShells.default =
            (buildFHSUserEnv
              {
                name = "fhs-shell";
                targetPkgs = pkgs:
                  with pkgs; [
                    rustup
		    clang
                    wayland
                    libxkbcommon
                    libGL
                  ];
              })
            .env;
        }
    );
}
