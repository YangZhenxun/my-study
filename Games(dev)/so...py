from ursina import *
from ursina.prefabs.sky import Sky
from ursina.prefabs.first_person_controller import FirstPersonController
app = Ursina()
grass = load_texture("Grass.jpg")


class Ground(Button):
    def __init__(self, position=(0, 0, 0)):
        super().__init__(
            parent=scene,
            module="cube",
            texture=grass,
            highlight_color=color.lime,
            color=color.white,
            position=position,
            origin_y=0.5
        )


height = 1
for y in range(0, height):
    for z in range(-15, 16):
        for x in range(-15, 16):
            print(f"Position:({x},{y},{z})")
            Ground(position=(x, y, z))
player = FirstPersonController()
sky = Sky()
app.run()