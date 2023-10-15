{ lib, fetchgit, rustPlatform }:
rustPlatform.buildRustPackage rec {
	pname = "daisy";
	version = "1.1.6";
	cargoLock.lockFile = src + /Cargo.lock;

	src = fetchgit {
		url = "https://github.com/rm-dr/daisy.git";
		rev = "v${version}";
		sha256 = "";
	};

	meta = with lib; {
		description = "A pretty command-line scientific calculator";
		homepage = "https://github.com/rm-dr/daisy";
		#license = licenses.GPL;
		maintainers = [ maintainers.tailhook ];
	};
}