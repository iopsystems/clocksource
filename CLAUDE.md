# Clocksource Development Guide

## Release Process

The release workflow in this repo requires **squash-merging** release PRs to trigger the automated tagging and publishing pipeline.

### Why squash merge?

The `tag-release.yml` workflow checks if the merge commit message contains `release: v`. With a **squash merge** of a PR titled `release: v1.0.1`, the merge commit message becomes:
```
release: v1.0.1 (#5)
```

With a **default merge commit**, the message is:
```
Merge pull request #5 from owner/branch

release: v1.0.1
```

The workflow guard `if: contains(github.event.head_commit.message, 'release: v')` matches squash-merge messages immediately but fails on merge-commit messages (where the title is on a later line).

### Release convention

1. Create a release PR with:
   - Title: `release: vX.Y.Z`
   - Single commit message: `release: vX.Y.Z`
   - Version bump in `Cargo.toml`
   - Updated `CHANGELOG.md`

2. **Always squash-merge** when merging to `main`
   - GitHub's merge button: select "Squash and merge"
   - Command line: ensure the merge commit message starts with `release: v`

3. After merge, `tag-release.yml` automatically:
   - Creates and pushes git tag `vX.Y.Z`
   - Triggers `release.yml` → CI + `cargo publish` → GitHub Release
   - Opens PR bumping to next dev version (`vX.Y.(Z+1)-alpha.0`)

If you accidentally merge without squashing, manually push the tag:
```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z" <merge-commit-sha>
git push origin vX.Y.Z
```

Then open a PR to bump the version to the next dev release.
