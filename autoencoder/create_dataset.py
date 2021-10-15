# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %% [markdown]
# Code ist im Rahmen der Bachelorarbeit von Annika Kies am FZI Karlsruhe mit dem Titel
# "Entwicklung und Analyse von Auto-Encodern für intelligente Agenten zum Erlernen von Atari-Spielen"
# entstanden.
# Code dient zur Erstellung von Datensätzen in einer .npy-Datei.
# Datensatz liegt an sich schon in einem Ordner vor, hier wird nur die Umwandlung von Bilder in eine .npy-Datei vorgenommen.
# Hier sind die Datensätze Q*Bert-Graustufen und Space Invaders - Graustufen gezeigt.

# %%
from PIL import Image
from numpy import asarray
import os
from numpy import save

"""
Funktion iteriert über jedes Bild im Ordner und wandelt es in ein Array um.
Array wird dann dem Testdatensatz hinzugefügt.
"""
def create_arrayTest(folder):
    for img_path in os.listdir(folder):
        path = os.path.join(folder, img_path)
        img = Image.open(path)
        img_array = asarray(img)  # convert to array
        test_data.append(img_array)  # add this to our training_data

"""
Funktion speichert einen Datensatz, mit Pfadangabe.
"""
def save_dataset(pathName, data):
    save(pathName, data)

if __name__ == '__main__':
    test_data = []
    create_arrayTest('images_small_size_big')
    print(len(test_data))
    print(test_data[0].shape)

    save_dataset('images_small_size_big.npy', test_data)
