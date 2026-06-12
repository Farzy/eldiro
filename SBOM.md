# Software Bill of Materials (SBOM) & Vulnerability Scanning

This project integrates **Syft** and **Grype** (by Anchore) to support Software Bill of Materials (SBOM) generation and security vulnerability scanning.

- **Syft**: A CLI tool and library for generating a Software Bill of Materials (SBOM) from container images and filesystems.
- **Grype**: An easy-to-use vulnerability scanner for container images and filesystems, designed to work seamlessly with Syft.

---

## Local Installation

You can install these tools locally to generate and scan SBOMs before committing code.

### macOS (via Homebrew)
```bash
brew install syft grype
```

### Linux
```bash
# Install Syft
curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh -s -- -b /usr/local/bin

# Install Grype
curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b /usr/local/bin
```

---

## Local Usage

### 1. Generating SBOMs
To generate an SBOM of the current directory, run one of the following commands:

*   **CycloneDX JSON Format (Recommended for security tools)**:
    ```bash
    syft . -o cyclonedx-json=sbom.cyclonedx.json
    ```
*   **SPDX JSON Format (ISO Standard for compliance)**:
    ```bash
    syft . -o spdx-json=sbom.spdx.json
    ```

### 2. Scanning with Grype
Once you have generated an SBOM, you can scan it for vulnerabilities:

```bash
grype sbom.cyclonedx.json
```

Alternatively, you can scan the filesystem directly without generating an SBOM file first:

```bash
grype dir:.
```

To fail the command when vulnerabilities above a certain threshold (e.g., `high`) are found:

```bash
grype sbom.cyclonedx.json --fail-on high
```

## CI/CD Pipeline Integration

Our GitHub Actions workflow [.github/workflows/ci.yml](file://.github/workflows/ci.yml) includes a `sbom` job that runs automatically on pushes and pull requests:

1.  **Generation**: Generates both CycloneDX JSON and SPDX JSON SBOM files.
2.  **Artifact Storage**: Uploads the generated SBOM files as run artifacts named `sbom-files`.
3.  **Vulnerability Enforcement**: Scans the generated CycloneDX SBOM with Grype and will fail the build if any `high` or `critical` vulnerabilities are discovered.

### Release CD Pipeline & Version Coherence

The CD workflow [.github/workflows/release.yml](file://.github/workflows/release.yml) is triggered automatically when you push a new version tag (with format `vX.Y.Z`, e.g., `v0.1.0`):

1.  **Version Coherence Check**: The workflow parses [Cargo.toml](file://Cargo.toml) and compares the `version` field with the Git tag name. If they do not match, the release job fails to prevent mislabeled or out-of-sync releases.
2.  **SBOM Generation**: Generates CycloneDX and SPDX SBOM formats for the tagged commit.
3.  **GitHub Release Creation**: Automatically publishes a new GitHub Release for the tag, generates release notes based on git commit history, and attaches the SBOM files as release assets.
