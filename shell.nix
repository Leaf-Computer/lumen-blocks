{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [    
    # Just command runner
    just
    
    # Uncomment the line below if you prefer to use the `dx` version from the nixpkgs
    # Please note that using a `dx` version lower than the `dioxus` dependency may cause issues
    # dioxus-cli
  ];
}
