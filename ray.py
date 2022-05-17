"""

"""

import numpy as np
import matplotlib.pyplot as plt
import logging

# List of objects.
C_WHITE = 1. * np.ones(3)
C_BLACK = 0. * np.ones(3)
C_BLUE = np.array([0., 0., 1.])
C_RED = np.array([1., 0., 0.])

C_PURPLE = np.array([.5, .223, .5])


def normalize(x):
    x /= np.linalg.norm(x)
    return x


def rotation_matrix(axis, theta):
    """
    Return the rotation matrix associated with counterclockwise rotation about
    the given axis by theta radians.
    """
    axis = axis / np.sqrt(np.dot(axis, axis))
    a = np.cos(theta / 2.0)
    b, c, d = -axis * np.sin(theta / 2.0)
    aa, bb, cc, dd = a * a, b * b, c * c, d * d
    bc, ad, ac, ab, bd, cd = b * c, a * d, a * c, a * b, b * d, c * d
    return np.array([[aa + bb - cc - dd, 2 * (bc + ad), 2 * (bd - ac)],
                     [2 * (bc - ad), aa + cc - bb - dd, 2 * (cd + ab)],
                     [2 * (bd + ac), 2 * (cd - ab), aa + dd - bb - cc]])


def intersect_plane(O, D, P, N):
    """
    Return the distance from O to the intersection of the ray (O, D) with the
    plane (P, N), or +inf if there is no intersection.
    O and P are 3D points, D and N (normal) are normalized vectors.
    :param O: ray origin
    :param D: ray direcction
    :param P: plane position
    :param N: plane normal
    :returns: distance to the intersection
    """

    denom = np.dot(D, N)
    if np.abs(denom) < 1e-6:
        return np.inf
    dist = np.dot( P - O, N) / denom
    if dist < 0:
        return np.inf
    return dist

def intersect_sphere(O, D, S, R):
    # Return the distance from O to the intersection of the ray (O, D) with the 
    # sphere (S, R), or +inf if there is no intersection.
    # O and S are 3D points, D (direction) is a normalized vector, R is a scalar.
    a = np.dot(D, D)
    OS = O - S
    b = 2 * np.dot(D, OS)
    c = np.dot(OS, OS) - R * R
    disc = b * b - 4 * a * c
    if disc > 0:
        distSqrt = np.sqrt(disc)
        q = (-b - distSqrt) / 2.0 if b < 0 else (-b + distSqrt) / 2.0
        t0 = q / a
        t1 = c / q
        t0, t1 = min(t0, t1), max(t0, t1)
        if t1 >= 0:
            return t1 if t0 < 0 else t0
    return np.inf

def intersect(O, D, obj):
    if obj['type'] == 'plane':
        return intersect_plane(O, D, obj['position'], obj['normal'])
    elif obj['type'] == 'sphere':
        return intersect_sphere(O, D, obj['position'], obj['radius'])

def get_normal(obj, M):
    # Find normal.
    if obj['type'] == 'sphere':
        N = normalize(M - obj['position'])
    elif obj['type'] == 'plane':
        N = obj['normal']
    return N
    
def get_color(obj, M):
    color = obj['color']
    if not hasattr(color, '__len__'):
        color = color(M)
    return color

def trace_ray(scene,rayO, rayD ):
    "scene, L ,ambient, diffuse, specular_c,specular_k "
    # Find first point of intersection with the scene.
    t = np.inf
    for i, obj in enumerate(scene.objects):
        t_obj = intersect(rayO, rayD, obj)
        if t_obj < t:
            t, obj_idx = t_obj, i
    # Return None if the ray does not intersect any object.
    if t == np.inf:
        return
    # Find the object.
    obj = scene.objects[obj_idx]
    # Find the point of intersection on the object.
    M = rayO + rayD * t
    # Find properties of the object.
    N = get_normal(obj, M)
    color = get_color(obj, M)

    toO = normalize(rayO - M)

    # Start computing the color.
    col_ray = color * .1
    for light in scene.lights:
        toL = normalize(light["pos"] - M)
        # Shadow: find if the point is shadowed or not.
        l = [intersect(M + N * .0001, toL, obj_sh)
                for k, obj_sh in enumerate(scene.objects) if k != obj_idx]
        if l and min(l) < np.inf:
            continue

        # Lambert shading (diffuse).
        col_ray += obj.get('diffuse_c') * max(np.dot(N, toL), 0) * color
        # Blinn-Phong shading (specular).
        col_ray += obj['specular_c'] * np.max(np.dot(N, normalize(toL + toO)), 0) ** obj['specular_k'] * light["color"]
    return obj, M, N, col_ray


def raycalc(scene, rayO,rayD,depth_max=3):
    reflection = 1.
    col = np.zeros(3)
    depth = 0
    # Loop through initial and secondary rays.
    while depth < depth_max:
        traced = trace_ray(scene,rayO, rayD )
        if not traced:
            break
        obj, M, N, col_ray = traced
        # Reflection: create a new ray.
        rayO, rayD = M + N * .0001, normalize(rayD - 2 * np.dot(rayD, N) * N)
        depth += 1
        col += reflection * col_ray
        reflection *= obj.get('reflection', 1.)

    return col
def add_sphere(position, radius, color,reflection=.5):
    return dict(type='sphere', 
        position=np.array(position), 
        radius=np.array(radius),
        color=np.array(color), 
        reflection=reflection,
        diffuse_c=.75,
        specular_c=.5,
        specular_k=10)




def add_Sqplane(position, normal,color_plane0,color_plane1):
    return dict(type='plane', position=np.array(position), 
        normal=np.array(normal),
        color=lambda M: (color_plane0 
            if (int(M[0] * 2) % 2) == (int(M[2] * 2) % 2) else color_plane1),
        diffuse_c=.75, specular_c=.5, reflection=.25,specular_k=1.)

def add_plane(position, normal,color_plane0,
              diffuse_c=.75,
              specular_c=.5,
              specular_k=1.,
              reflection=.25):
    return dict(type='plane', position=np.array(position), 
        normal=np.array(normal),
        color=color_plane0 ,
        diffuse_c=diffuse_c ,
        specular_c=specular_c,
        specular_k=specular_k,
        reflection=reflection)

def add_light(pos,color,):
    return dict(type='light',pos=pos,color=color)

class Scene(object):

    def __init__(self,lights,objects,camera):
        self.lights= lights
        self.objects = objects
        self.camera = camera

    def setCamera(self,loc=None,dir=None):
        if loc is not None:
            self.camera[0] = loc
        if dir is not None :
            self.camera[1] = dir

def iterateGenerate(scene,w=10,h=10,depth_max=3,figname=None):
    """


    """
    log = logging.getLogger("it")

    O,Q = scene.camera

    rmat = rotation_matrix([0,1,0], np.pi/2.)
    camDir = normalize(Q - O)

    planeLoc = O+camDir
    orthx = rmat.dot(camDir)

    orthy = rotation_matrix(camDir, np.pi / 2.).dot(orthx)
    log.info("camDir: %s  orth: %s", camDir, orthx)

    r = float(w) / h
    # Screen coordinates: x0, y0, x1, y1.
    S = (-1., -1. / r + .25, 1., 1. / r + .25)

    img = np.zeros((h, w, 3))

    col = np.zeros(3)  # Current color.
    # Loop through all pixels.
    for i, x in enumerate(np.linspace(S[0], S[2], w)):
        if i % 10 == 0:
            print(i / float(w) * 100, "%")
        for j, y in enumerate(np.linspace(S[1], S[3], h)):
            col[:] = 0

            rayD = normalize((planeLoc + [x*orthx[0] , orthy[1]*y ,x*orthx[2] ]) - O)
            rayO = O

            col = raycalc(scene, rayO, rayD, depth_max=depth_max )

            img[h - j - 1, i, :] = np.clip(col, 0, 1)

    if figname is None:
        return img
    else:
        plt.imsave(figname, img)


