{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in pkgs.stdenv.mkDerivation {
  name = manifest.name;
  version = manifest.version;

  nativeBuildInputs = [
    pkgs.cargo
    pkgs.dosbox
    pkgs.nasm
  ];

  WATCOM=pkgs.open-watcom-bin;
  APPNAME=manifest.name;

  shellHook=''
    build() {
      cargo clean
      cargo build

      export APPFILE=$(echo $APPNAME | sed s/-/_/)

      printf "segment stack use32 class=stack\n\tresb 8000h" > .stack.s
      nasm -f obj .stack.s -o .stack.o

      export PATH=$PATH:$WATCOM/binl:$WATCOM/binw

      wlink name target/dos/debug/$APPFILE.exe system dos32a option dosseg option start=_dtg_entrypoint file .stack.o library target/dos/debug/lib$APPFILE.a

      rm .stack.s
      rm .stack.o
    }

    run() {
      build
      dosbox target/dos/debug/$APPFILE.exe
    }
  '';
}
