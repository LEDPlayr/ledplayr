{ pkgs, ... }: {
  env = { CROSS_CUSTOM_TOOLCHAIN = "1"; };

  packages = with pkgs; [ pre-commit sqlite.dev ];

  languages.rust = {
    enable = true;
    channel = "stable";

    components =
      [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-src" ];

    targets = [ "x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" ];
  };
}
