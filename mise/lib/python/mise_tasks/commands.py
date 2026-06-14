"""Logged subprocess helpers for mise task scripts."""

import logging
import subprocess
from typing import Sequence

log = logging.getLogger(__name__)


def run(cmd: Sequence[str], **kwargs) -> subprocess.CompletedProcess:
    """Run a command, logging it first. Raises on non-zero exit."""
    log.info("+ %s", " ".join(cmd))
    return subprocess.run(list(cmd), check=True, **kwargs)


def capture(cmd: Sequence[str]) -> str:
    """Run a command and return its stripped stdout."""
    log.info("+ %s", " ".join(cmd))
    result = subprocess.run(list(cmd), check=True, capture_output=True, text=True)
    return result.stdout.strip()
