# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %%
# from IPython import get_ipython

# %%
# get_ipython().run_line_magic('load_ext', 'autoreload')
# get_ipython().run_line_magic('autoreload', '2')


# %%
import itertools
import os

import torch
from numpy import load
import numpy as np
from torch import nn
import matplotlib.pyplot as plt
import GuiImageDataset
from torch.utils.tensorboard import SummaryWriter
import AutoencoderSmallerImages3Channels
from torch_service import plot_classes_preds

# %%
# Tensorboard setup
writer = SummaryWriter()

# %%
def to_img(x):
    x = 0.5 * (x + 1)
    x = x.clamp(0, 1)
    x = x.view(x.size(0), 1, 28, 28)
    return x


# %%
"""
Funktion lädt den Datensatz
"""
def load_dataset(path):
    return load(path)


def load_paths_from_folder(folder_name):
    paths = []
    for path in os.listdir(folder_name):
        paths.append(folder_name + "/" + path)
    return paths

"""
Funktion normalisiert einen Pixelwert
"""
def normalize(v):
    return v / 255


# %%
paths = load_paths_from_folder("datasets/images_very_big")


# %%
# trn_dataloader = torch.utils.data.DataLoader(trn, batch_size=1, shuffle=False, num_workers=0)

transformed_dataset = GuiImageDataset.LazyLoadedGuiImageDataset(paths)
dataset_loader =  torch.utils.data.DataLoader(transformed_dataset, batch_size=16, num_workers=0)

print("Training Datensatz:")
print(len(paths))
print(transformed_dataset.__getitem__(0).shape)
# dataloader = DataLoader(dataset, batch_size=batch_size, shuffle=True)


# %%
# print random images
#Utility functions to un-normalize and display an image
def imshow(img):
    img = img / 2 + 0.5
    plt.imshow(np.transpose(img, (1, 2, 0)))

#Obtain one batch of training images
dataiter = iter(dataset_loader)
images = dataiter.next()
images = images.numpy() # convert images to numpy for display

#Plot the images
fig = plt.figure(figsize=(8, 8))
# display 1 image
for idx in np.arange(1):
    ax = fig.add_subplot(3, 3, idx+1, xticks=[], yticks=[])
    imshow(images[idx])


# %%
model = AutoencoderSmallerImages3Channels.Autoencoder2VAEMediumConvBigKernel()
# model.load_state_dict(torch.load("run_156cfdf1c3f95af1b0631200ac2e4f83187842e8.pt"))

learning_rate = 2e-3

criterion = nn.BCELoss()
optimizer = torch.optim.Adamax(
    model.parameters(), lr=learning_rate
)


# %%
def get_device():
    if torch.cuda.is_available():
        device = 'cuda:0'
    else:
        device = 'cpu'
    return device

device = get_device()
print(device)
model.to(device)


# %%
#Epochs
n_epochs = 2
log_rhythm = 25

for epoch in range(1, n_epochs+1):
    # monitor training loss
    train_loss = 0.0
    running_loss = 0.0
    i=0
    print(len(dataset_loader))
    #Training
    # for data in itertools.islice(dataset_loader, 2000):
    for data in dataset_loader:
        print('Batch Index: {}'.format(i))
        i+=1
        images = data
        # images = torch.autograd.Variable(images)
        images = images.to(device)
        optimizer.zero_grad()
        outputs = model(images)
        loss = criterion(outputs, images)
        loss.backward()
        optimizer.step()
        train_loss += loss.item()

        running_loss += loss.item()
        if i % log_rhythm == log_rhythm - 1:    # every n batches...
            # ...log the running loss
            writer.add_scalar('training loss',
                            running_loss / log_rhythm,
                            i)

            print("Current Loss: {}".format(train_loss/i))

            # ...log a Matplotlib Figure showing the model's predictions on a
            # random mini-batch
            writer.add_figure('predictions vs. actuals',
                            plot_classes_preds(model, images, outputs),
                            i)
            model.to(device)
            running_loss = 0.0


    train_loss = train_loss/len(dataset_loader)
    print('Epoch: {} \tTraining Loss: {:.6f}'.format(epoch, train_loss))


# %%
print(train_loss/750*len(dataset_loader))
torch.save(model.state_dict(), "autoencoder_test.pt")


