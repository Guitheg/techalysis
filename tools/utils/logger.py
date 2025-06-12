import logging
import sys

def initLogger():
    fmt = "({levelname}) {message}"

    handler = logging.StreamHandler(sys.stderr)
    handler.setFormatter(logging.Formatter(fmt, style="{"))

    logging.basicConfig(
        level=logging.DEBUG,
        handlers=[handler],
    )
initLogger()
logger = logging.getLogger("TechalysisPyTools")
