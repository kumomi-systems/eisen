let
  shellScripts = pkgs.lib.strings.concatMapStrings (t: " " + t) [
    "./kernel/gen-env-linker.sh"
    "./kernel/sign.sh"
    "./kernel/strip.sh"
  ];
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    toolchain
    rust-analyzer-unwrapped
    llvmPackages.bintools
    gptfdisk
    rhash
    qemu_full
    OVMF.fd
    tinyxxd
  ];

  RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

  shellHook = ''
    for f in ${shellScripts}; do
      chmod +x $f
    done

    rustup target add x86_64-unknown-uefi
    rustup target add x86_64-unknown-none

    if [ ! -d OVMF ]; then mkdir OVMF; fi
    cp -r ${pkgs.OVMF.fd}/FV/OVMF.fd OVMF/OVMF.4m.fd
    chmod -R 755 OVMF
    chown -R --reference=. OVMF
  '';
}