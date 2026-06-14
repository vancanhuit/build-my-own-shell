"""Release helpers shared by the release:* mise tasks."""

import logging
import os
import subprocess
import tarfile
from pathlib import Path
from typing import List, Optional

from . import commands

log = logging.getLogger(__name__)

BIN_NAME = "shell"


def project_root() -> Path:
    """Return the repository root provided by mise."""
    root = os.environ.get("MISE_PROJECT_ROOT")
    if not root:
        raise SystemExit("MISE_PROJECT_ROOT is not set. Run this task through mise.")
    return Path(root)


def resolve_tag(arg_tag: Optional[str]) -> str:
    """Resolve the release tag from a CLI argument or RELEASE_TAG env var."""
    tag = arg_tag or os.environ.get("RELEASE_TAG")
    if not tag:
        raise SystemExit("No tag provided. Use --tag <tag> or set RELEASE_TAG.")
    return tag


def host_triple() -> str:
    """Return the host target triple reported by rustc."""
    for line in commands.capture(["rustc", "-vV"]).splitlines():
        if line.startswith("host:"):
            return line.split(":", 1)[1].strip()
    raise SystemExit("Could not determine host target triple from 'rustc -vV'.")


def run_ci() -> None:
    """Run the full local CI suite before producing artifacts."""
    log.info("Running rust:ci before packaging")
    commands.run(["mise", "run", "rust:ci"])


def build_release() -> Path:
    """Build the optimized binary and return its path."""
    commands.run(["cargo", "build", "--release"])
    binary = project_root() / "target" / "release" / BIN_NAME
    if not binary.is_file():
        raise SystemExit(f"Expected release binary not found: {binary}")
    return binary


def package(tag: str) -> Path:
    """Run CI, build the release binary, and package it into a tarball."""
    run_ci()
    binary = build_release()
    triple = host_triple()
    dist = project_root() / "dist"
    dist.mkdir(parents=True, exist_ok=True)
    artifact = dist / f"{BIN_NAME}-{tag}-{triple}.tar.gz"
    log.info("Packaging %s into %s", binary.name, artifact)
    with tarfile.open(artifact, "w:gz") as tar:
        tar.add(binary, arcname=binary.name)
    return artifact


def verify_tag(tag: str) -> None:
    """Ensure the git tag exists locally before publishing."""
    try:
        commands.run(
            ["git", "rev-parse", "-q", "--verify", f"refs/tags/{tag}"],
            stdout=subprocess.DEVNULL,
        )
    except subprocess.CalledProcessError as exc:
        raise SystemExit(
            f"Tag not found: {tag}. Create and push it before releasing."
        ) from exc


def previous_tag(tag: str) -> Optional[str]:
    """Return the tag immediately preceding `tag`, or None if it is the first."""
    try:
        return commands.capture(
            ["git", "describe", "--tags", "--abbrev=0", f"{tag}^"]
        )
    except subprocess.CalledProcessError:
        return None


def generate_notes(tag: str) -> str:
    """Generate release notes for `tag` using Cocogitto's changelog.

    Covers commits since the previous tag, falling back to the full history
    for the first release. The leading version header is stripped so the notes
    contain only the grouped changelog sections.
    """
    prev = previous_tag(tag)
    cmd = ["cog", "changelog"]
    if prev:
        cmd.append(f"{prev}..{tag}")
    output = commands.capture(cmd)
    lines = output.splitlines()
    if lines and lines[0].startswith("## "):
        lines = lines[1:]
    return "\n".join(lines).strip()


def release_command(tag: str, title: str, notes: Optional[str], artifact: Path) -> List[str]:
    """Build the 'gh release create' command for the given inputs."""
    cmd = ["gh", "release", "create", tag, str(artifact), "--title", title]
    if notes:
        cmd += ["--notes", notes]
    else:
        cmd += ["--generate-notes"]
    return cmd


def create_release(tag: str, title: str, notes: Optional[str], artifact: Path) -> None:
    """Publish a GitHub release with the packaged artifact."""
    if not notes:
        notes = generate_notes(tag)
    commands.run(release_command(tag, title, notes, artifact))
