

import logging
import ray
import numpy as np
import unittest
import matplotlib.pyplot as plt


class RayCalc_Tests(unittest.TestCase):

    def test_empty_scene(self):
        scene = testScene1 ()

        rayO = np.array([0,0,0])
        rayD = np.array([1,0,0])


        L = np.array([5., 5., -10.])

        ambient = .05
        res = ray.raycalc(scene, rayO, rayD )

        #print(res)



def testScene1 ():
    return ray.Scene(
        [
            ray.add_light(np.array([5., 5., -10.]),np.array([1., 1., 1.])),
            ray.add_light(np.array([-4., 1.5, -3.]), np.array([1., 1., 1.])),
         ],
        [
         ray.add_sphere([3, 0, 0], .6, ray.C_RED, .0),
         ray.add_sphere([0, 0, 3], .6, ray.C_WHITE, .8),
         ray.add_sphere([.75, .1, 1.], .6, ray.C_BLUE, .0),
         ray.add_sphere([-.75, .1, 2.25], .3, ray.C_PURPLE),
         ray.add_sphere([-2.75, .1, 3.5], .6, [1., .572, .184]),
         ray.add_plane([0., -.5, 0.], [0., 1., 0.], ray.C_WHITE,
                       diffuse_c = .9,
                       specular_c=.01,
                       specular_k = .0050 ,
                       reflection = .0),
         ],
        [np.array([-5., 0., 3.]),
         np.array([1., 0., 0.])],
    )
class IterateGenerate_Tests(unittest.TestCase):
    def test1_simple(self):
        scene = testScene1 ()
        w = int(150)
        h = int(w * 9.0 / 16.0)
        rayO = np.array([0., 0., 0.])
        rayD = np.array([1., 0., 0.])
        scene.setCamera(rayO,rayD)
        ray.iterateGenerate(scene,figname="t1.png",w=w, h=h)


    def test2_simple(self):
        logging.basicConfig(level=logging.INFO)
        scene = testScene1 ()


        w = int(150)
        h = int(w * 9.0 /16.0)
        dirs = [

            ray.normalize(np.array([1, .0  ,0.0])),
            ray.normalize(np.array([1, .0, 1])),
            ray.normalize(np.array([1.0, 0.5, 0.])),
            #np.array([3, 0, 0]),
            #np.array([-2.75, .1, 3.5]),
            ]
        finalimg = np.zeros((h, w * len(dirs), 3))

        for dirid,dir in enumerate(dirs):

            scene.setCamera(dir = dir)
            img = ray.iterateGenerate( scene, w=w, h=h)

            finalimg[:,dirid*w:(dirid+1)*w,:] = img

        plt.imsave("t2.png", finalimg)
