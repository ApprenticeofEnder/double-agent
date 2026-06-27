{pkgs, ...}: {
  packages = with pkgs; [
    git
    jujutsu
    lazyjj
  ];

  languages = {
    rust = {
      enable = true;
    };
  };
}
