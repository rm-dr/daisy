{ lib, fetchgit, rustPlatform }:
rustPlatform.buildRustPackage rec {
	pname = "daisy";
	version = "1.1.7";
	cargoLock.lockFile = src + /Cargo.lock;

	src = builtins.fetchGit {
		url = "https://github.com/rm-dr/daisy.git";
		ref = "refs/tags/v${version}";
		#rev = ""; Ideally, we'd have a hash here, but that would make git history messy.
	};

	meta = with lib; {
		description = "A general-purpose scientific calculator";
		homepage = "https://github.com/rm-dr/daisy";
		#license = licenses.GPL;
		maintainers = [ maintainers.tailhook ];
	};
}