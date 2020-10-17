import json
import numpy as np

from kafka import KafkaProducer
from kafka import KafkaConsumer

def bytify(msg):
    return json.dumps( msg).encode("utf-8")
        
def unbytify(data):
    return json.loads(data.decode("utf-8"))

class Net():

    def __init__(self):
        bss = "192.168.1.80:9092"
    
        self.producer = KafkaProducer(bootstrap_servers=bss)
        self.consumer = KafkaConsumer("pixval",bootstrap_servers=bss)
        
        self.lastPos,self.lastLA = None,None

    def update(self,game):
        if self.lastPos is None:
            self.sendCamData(game)
        elif not np.array_equal(self.lastPos, game.camera.pos) :
            self.sendCamData(game)
        elif not np.array_equal(self.lastLA , game.camera.lookat):
            self.sendCamData(game)
        
        buff = self.consumer.poll()
        
        if len(buff)>0:
            for topicpart,records in buff.items():
                game.vscreen.lock()
                for record in records:
                    msg = unbytify( record.value)
                    print(topicpart.topic,msg)
                    game.vscreen.set_at((msg["wid"],msg["hid"]),msg["color"])
                game.vscreen.unlock()
    def sendCamData(self,game): 
        """

        """
        data = bytify(dict(
            type="camdata",
            pos=game.camera.pos.tolist(), 
            lookat=game.camera.lookat.tolist(),
            slideat = game.camera.lookat.tolist()    
            ))
        self.producer.send("campos", data )
        self.producer.flush()
        self.lastPos = game.camera.pos
        self.lastLA = game.camera.lookat
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


class Camera():
    def __init__(self):

        self.pos = np.array([0,0,0])
        self.lookat = np.array([1,0,0])
        self.slideat = np.array([0,1,0])
    
    def rotate(self,zrot,vrot):
        rmat= rotation_matrix(np.array([0,0,1]),zrot)
        self.lookat=rmat.dot(self.lookat)
        self.slideat=rmat.dot(self.slideat)

        rmat= rotation_matrix(self.slideat,vrot)
        self.lookat=rmat.dot(self.lookat)
        self.slideat=rmat.dot(self.slideat)

    def translate(self,tx,ty,tz):
        x,y,z = pos
        self.pos = np.array([x+tx,y+ty,z+tz])



class Game():

    def __init__(self):
        
        pygame.init()
        self.screenRes = (468, 300)
        self.screen = pygame.display.set_mode(self.screenRes)

        self.vscreen = pygame.Surface((10,6))

        pygame.display.set_caption('Sparkcast Renderer')
        pygame.mouse.set_visible(0)
        pygame.event.set_grab(True)
        

        self.background = pygame.Surface(self.screen.get_size())
        self.background = self.background.convert()

        self.clock = pygame.time.Clock()
        self.active=True
        
        self.camera = Camera()
        self.network = Net()
        self.mousePos = (0,0)
        self.rotSpeed = .005 
    def update(self):
        """
        """
        self.clock.tick(30)

        self.network.update(self)

        for event in pygame.event.get():
            if event.type == QUIT:
                print("quit")
                self.active=False
            elif event.type == KEYDOWN and event.key == K_ESCAPE:
                print("escape")
                self.active=False
            elif event.type == pygame.MOUSEMOTION:
                print("mouse %s"%event)
                self.mousePos = event.pos
                self.camera.rotate(event.rel[0]*self.rotSpeed,event.rel[1]*self.rotSpeed)

            elif event.type == KEYDOWN and event.key == K_LEFT:
                print("left!")

    def render(self):

        color_line= (255,0,0)
        #cale(Surface, (width, height), DestSurface = None) -> Surface

        pygame.transform.scale(self.vscreen,self.screenRes,self.background   )

        #self.background.fill((50, 50, 50))


        # draw line         
        ornp = np.array([130,100]) 
        to = (ornp+self.camera.lookat[:2]*40).tolist()
        pygame.draw.line(self.background,color_line, to,ornp) 
        
        self.screen.blit(self.background,(0,0))

        pygame.display.flip()

if __name__=="__main__":
    import pygame
    from pygame.locals import *
    game = Game()

    while game.active:
        game.update()
        if not game.active: break
        game.render()
    


