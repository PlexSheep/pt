"""
test the logger
"""
import unittest
from libpt import logger

class TestLogger(unittest.TestCase):

    def test_basic_logging(self):
        logger.Logger.init()
        l = logger.Logger()
        l.trace("MSG")
        l.debug("MSG")
        l.info("MSG")
        l.warn("MSG")
        l.error("MSG")

if __name__ == '__main__':
    unittest.main()


