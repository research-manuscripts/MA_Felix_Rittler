import torch
import matplotlib.pyplot as plt

def get_device():
    if torch.cuda.is_available():
        device = 'cuda:0'
    else:
        device = 'cpu'
    return device

"""
Funktion zeigt ein Bild
"""
def show_torch_image(torch_tensor):
    plt.figure(figsize=(100,100))
    plt.imshow(torch_tensor)
    plt.show()