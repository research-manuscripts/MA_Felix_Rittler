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
import random

"""
Funktion iteriert über jedes Bild im Ordner und wandelt es in ein Array um.
Array wird dann dem Testdatensatz hinzugefügt.
"""
def create_paths(folder):
    for img_path in os.listdir(folder):
        paths.append(img_path)


"""
Funktion speichert einen Datensatz, mit Pfadangabe.
"""
def save_dataset(pathName, data):
    save(pathName, data)

if __name__ == '__main__':
    paths = []

    create_paths('images_small_size_big')

    save_dataset('big_dataset_paths.npy', paths)


