# Release Manifest Signing Protocol

A release is not final until the manifest and distribution archives are hashed and signed.

## Required files

```text
release_manifest.json
SCC_Kernel_Fortress_<tag>_checksums.sha256
SCC_Kernel_Fortress_<tag>_checksums.sha256.asc
```

## Commands

```bash
sha256sum *.zip *.pdf *.tex *.txt > SCC_Kernel_Fortress_<tag>_checksums.sha256
gpg --armor --detach-sign SCC_Kernel_Fortress_<tag>_checksums.sha256
gpg --verify SCC_Kernel_Fortress_<tag>_checksums.sha256.asc SCC_Kernel_Fortress_<tag>_checksums.sha256
sha256sum -c SCC_Kernel_Fortress_<tag>_checksums.sha256
```

## Evidence rule

Do not write that a release is signed unless the detached signature file is present and verification passes. If a sandbox cannot access the signing key, record that limitation and require a maintainer-signed release transcript.
