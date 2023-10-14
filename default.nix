{ lib, fetchgit, rustPlatform }:
rustPlatform.buildRustPackage rec {
	pname = "daisy";
	version = "1.1.4";
	cargoLock.lockFile = src + /Cargo.lock;

	src = fetchgit {
		url = "https://github.com/rm-dr/daisy.git";
		rev = "7658ff76ef2d2fab540dc8b8d4ee24077daebd01";
		sha256 = "sha256-aENuKtE1+tBRN0HZzRr8Gk+dVEYTiP6FNRz817Sk88o=";
	};

	meta = with lib; {
		description = "A pretty command-line scientific calculator";
		homepage = "https://github.com/rm-dr/daisy";
		license = licenses.GPL;
		maintainers = [ maintainers.tailhook ];
	};
}