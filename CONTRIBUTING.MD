# Contribution guidelines for Daisy


## Pull requests
All PRs should merge to `dev`. See "branching" below.


## Versioning
Daisy uses [semantic versioning](https://semver.org/), which looks like `maj.min.patch`. In short:
 - `maj`: Incremented on large, incompatible changes
 - `min`: Incremented on new features
 - `patch`: Incremented on bugfixes

Whenever one of the above fields is incremented, all sub-fields are reset to zero. When `min` is incremented, `patch` is set to zero.


## Branching
Daisy uses a branch model much like the one described [here](https://nvie.com/posts/a-successful-git-branching-model).
 - `master`: 1 commit = 1 release. This is always a merge comit from `dev`, and is always tagged with a version.
 - `dev`: main development branch. Never deleted. Most work happens here.
 - `features`: temporary branches pulled from `dev` and merged to `dev` for larger features.
 - Always merge with git's `--no-ff` option for a more detailed history.


## Releases
What to do 
 - Merge into `master` (`git merge --no-ff dev`)
 - Tag merge commit on `master` (`git tag -a v1.0.0 -m "Version 1.0.0"`)
 - `cargo publish`
 - Update web demo & pull server (`make docker`)
 - Update aur package
 - Update `default.nix` (test with `make nix`)