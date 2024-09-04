{ rustPlatform, fetchFromGitHub, lib}:

rustPlatform.buildRustPackage rec {
  pname = "gtnkr";
  version = "0.0.0";

  src = ../../.;
  cargoLock.lockFile = "${src}/Cargo.lock";

  useNextest = true;

  meta = {
    description = "gtnkr is a wrapper tool for use with Steam which allows you to easily configure tools like gamescope, mangohud and libstrangle";
    homepage = "https://github.com/bordomantra/gtnkr";
    license = lib.licenses.gpl3Only;
	platforms = [ "x86_64-linux" ];
    mainProgram = "gtnkr";
    maintainers = with lib.maintainers; [ bordomantra ];
  };
}
