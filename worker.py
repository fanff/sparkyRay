

from renderer import rotation_matrix,bytify,unbytify

from kafka import KafkaProducer
from kafka import KafkaConsumer

import numpy as np
import random

wres = 10
hres = 6
fov = 30

vfov = float(fov * hres) / wres

# buffer rotation matrix along z axis
zaxis = np.array([0,0,1])
zmats = [[angl ,rotation_matrix(zaxis,np.pi*2*angl/360.0)] for angl in np.arange(-fov/2,fov/2,fov/wres)]
    
consumer = KafkaConsumer('campos',bootstrap_servers='192.168.1.80:9092')

producer = KafkaProducer(bootstrap_servers='192.168.1.80:9092')
for data in consumer:
    pass
    msg = unbytify(data.value) 

    if msg["type"] == "camdata":
        pos = np.array(msg["pos"])
        lookat = np.array(msg["lookat"])
        slideat = np.array(msg["slideat"])
        for aid , (angl,rmat) in enumerate(zmats):
            rayDir = rmat.dot(lookat)
            slideDir = rmat.dot(slideat)

            for vaid , vangl in enumerate(np.arange(-vfov/2,vfov/2, vfov/wres)):
                vrmat = rotation_matrix(slideDir,vangl)
                rayDir = vrmat.dot(rayDir)
                
                data = bytify(dict(type="pixval",wid=vaid,hid=aid,color = (23,int(230*random.random()),23)))
                producer.send("pixval",data)
    else:
        print (msg)
        raise Exception("unknown message")

