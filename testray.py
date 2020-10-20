

import ray
import numpy as np
import unittest

class SimpleTest(unittest.TestCase):

    def test_1(self):
        scene = []

        rayO = np.array([0,0,0])
        rayD = np.array([1,0,0])


        L = np.array([5., 5., -10.])


        ray.raycalc(scene,  )
