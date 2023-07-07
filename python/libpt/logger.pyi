"""
# A specialized Logger for `libpt`
"""

""" the default log level """
DEFAULT_LOG_LEVEL = "INFO"
""" Set the value of this key as envar to set a loglevel """
LOGGER_ENV_KEY = "LIBPT_LOGLEVEL"

class Logger:
    """
    `libpt` logger

    Call `init` once before usage, else all logging attempts will be ignored.
    """
    def __init__(self):
        """
        get a new logger
        """
        ...

    @staticmethod
    def init():
        """
        initialize the logger before the first usage
        """
        ...

    @staticmethod
    def init_specialized(color: bool):
        """
        initialize the logger before the first usage, but with the ability to tweak things a bit
        """
        ...

    def error(self, printable: str):
        """
        log at level `error`
        """
        ...

    def warn(self, printable: str):
        """
        log at level `error`
        """
        ...

    def info(self, printable: str):
        """
        log at level `error`
        """
        ...

    def debug(self, printable: str):
        """
        log at level `error`
        """
        ...

    def trace(self, printable: str):
        """
        log at level `error`
        """
        ...

