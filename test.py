import pygame
from pygame.locals import *



class Game():

    def __init__(self):
        
        pygame.init()

        self.screen = pygame.display.set_mode((468, 300))

        pygame.display.set_caption('Sparkcast Renderer')
        pygame.mouse.set_visible(0)
        pygame.event.set_grab(True)

        self.background = pygame.Surface(self.screen.get_size())
        self.background = self.background.convert()

        self.clock = pygame.time.Clock()
        self.active=True
        

        self.mousePos = (0,0)
    def update(self):
        """
        """
        self.clock.tick(30)
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


    def render(self):

        color_line= (255,0,0)
        
        self.background.fill((50, 50, 50))
        
        pygame.draw.line(self.background,color_line, self.mousePos,(130, 100)) 
        
        self.screen.blit(self.background,(0,0))

        pygame.display.flip()

if __name__=="__main__":
    game = Game()

    while game.active:
        game.update()
        if not game.active: break
        game.render()
    


