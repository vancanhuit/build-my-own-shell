"""Traceable logging configuration for mise task scripts."""

import logging


def configure_logging(level: int = logging.INFO) -> None:
    """Configure root logging with a traceable, structured format."""
    logging.basicConfig(
        level=level,
        format=(
            "%(asctime)sZ %(levelname)s "
            "module=%(module)s "
            "file=%(filename)s "
            "function=%(funcName)s "
            "line=%(lineno)d "
            'message="%(message)s"'
        ),
        datefmt="%Y-%m-%dT%H:%M:%S",
    )
