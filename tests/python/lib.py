"""
tests for the general behaviour of the libraries availability
"""
import unittest
import libpt

class TestLibptGeneral(unittest.TestCase):

    def test_loaded(self):
        assert libpt.is_loaded()

if __name__ == '__main__':
    unittest.main()


