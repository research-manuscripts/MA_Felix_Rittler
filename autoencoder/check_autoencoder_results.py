# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %% [markdown]
# Tests an autoencoder by calculating loss and printing both original and generated image.

# %%
import torch
from matplotlib import pyplot as plt
from AutoencoderSmallerImages import Autoencoder2VAEMediumConv
import GuiImageDataset
import numpy as np
from torch_service import show_torch_image
from torch import nn
from numpy import load
import itertools


# %%
autoencoder_path = "trained_autoencoders/VAEMediumConv.pt"

model = Autoencoder2VAEMediumConv()


# %%
dataset = load("datasets/images_smaller_size.npy")
transformed_dataset = GuiImageDataset.GuiImageDataset(dataset)
dataset_loader =  torch.utils.data.DataLoader(transformed_dataset, batch_size=1, num_workers=0)


# %%
# print random images
#Utility functions to un-normalize and display an image
def imshow(img):
    plt.figure(figsize=(100,100))
    plt.imshow(np.transpose(img, (1, 2, 0)))

#Obtain one batch of training images
dataiter = iter(dataset_loader)
images = dataiter.next()
images = images.numpy() # convert images to numpy for display

#Plot the images
fig = plt.figure(figsize=(8, 8))
# display 1 image
for idx in np.arange(1):
    imshow(images[idx])


# %%
# Hier wird der trainierte Auto-Encoder geladen
model.load_state_dict(torch.load(autoencoder_path))
# ae.eval()

for data in itertools.islice(dataset_loader, 2):
    images = data
    pred = model(images)
    loss = nn.MSELoss()(pred, images)
    pred = pred.permute(0, 2, 3, 1)

    print(pred.shape)
    frame = images[0].detach().permute(1,2,0)
    print("Loss: ", loss.item())
    show_torch_image(frame)
    show_torch_image(pred[0].detach())



pred = pred.permute(0, 2, 3, 1)

print(pred.shape)
frame = images[0].detach().permute(1,2,0)
print("Loss: ", loss.item())
show_torch_image(frame)
show_torch_image(pred[0].detach())


