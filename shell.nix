{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # TailwindCSS CLI
    tailwindcss
    
    # Just command runner
    just
  ];
}
