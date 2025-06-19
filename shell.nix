{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # TailwindCSS CLI
    tailwindcss_4
    
    # Just command runner
    just
  ];
}
