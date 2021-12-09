import os
import numpy as np
import torch
import matplotlib.pyplot as plt

"""
Retrieve the currently available cuda device
"""
def get_device():
    if torch.cuda.is_available():
        device = 'cuda:0'
    else:
        device = 'cpu'
    return device

"""
List all paths in a folder
"""
def load_paths_from_folder(folder_name):
    paths = []
    for path in os.listdir(folder_name):
        paths.append(folder_name + "/" + path)
    return paths

"""
Show a image with matplotlib
"""
def show_torch_image(torch_tensor):
    plt.figure(figsize=(100,100))
    plt.imshow(np.transpose(torch_tensor, (1, 2, 0)))
    plt.show()

"""
Plots a prediction
"""
def plot_classes_preds(net, images, outputs):
    net = net.cpu()
    # plot the images in the batch, along with predicted and true labels
    fig = plt.figure(figsize=(12, 48))
    for idx in np.arange(images.size(0)):
        plt.imshow(np.transpose(outputs[idx].cpu().detach().numpy(), (1,2,0)))
    return fig
