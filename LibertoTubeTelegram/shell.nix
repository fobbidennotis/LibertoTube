{ pkgs ? import <nixpkgs> {} }: pkgs.mkShell {
  packages = [
    (pkgs.python312.withPackages (pypkgs: with pypkgs; [
      aiogram
      requests
    ]))
  ];
}
