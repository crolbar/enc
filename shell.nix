{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    nativeBuildInputs = with pkgs; [ 
        rustc 
        cargo
        gcc
        rustfmt
        clippy 
        glib 
        glibc 
        gtk3 
        gtk-layer-shell 
        pkg-config 
    ];
}
