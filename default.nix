{ lib, fetchgit, rustPlatform }:
rustPlatform.buildRustPackage rec {
	pname = "daisy";
	version = "1.1.4";
	cargoLock.lockFile = src + /Cargo.lock;

	src = fetchgit {
		url = "https://github.com/rm-dr/daisy.git";
		rev = "v${version}";
		sha256 = "sha256-aENuKtE1+tBRN0HZzRr8Gk+dVEYTiP6FNRz817Sk88o=";
	};

	meta = with lib; {
		description = "A pretty command-line scientific calculator";
		homepage = "https://github.com/rm-dr/daisy";
		#license = licenses.GPL;
		maintainers = [ maintainers.tailhook ];
	};
}