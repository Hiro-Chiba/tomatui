# Releasing

Tomatui releases use a Git tag, crates.io Trusted Publishing, and a matching GitHub Release. Prebuilt binaries and GitHub Packages are not part of the release process.

## One-time setup

Create a GitHub environment named `release` and require maintainer approval.

In the `tomatui` settings on crates.io, add a GitHub trusted publisher with these values.

- Repository owner `Hiro-Chiba`
- Repository name `tomatui`
- Workflow filename `release.yml`
- Environment name `release`

This setup gives the workflow a short-lived publishing token and avoids storing a crates.io token in GitHub Secrets.

## Release process

Update the version in `Cargo.toml` and `Cargo.lock`, then add the release notes and date to `CHANGELOG.md`. Commit the changes to `main` and wait for CI to pass.

Create and push an annotated tag that matches the Cargo version.

```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

Approve the `release` environment deployment. The workflow verifies the tag and version, runs checks on all supported operating systems, packages and publishes the crate, and creates the GitHub Release. The release includes the Cargo install command, highlights from `CHANGELOG.md`, and GitHub's generated change list.
