import torch
import numpy as np
from torch import nn
import GuiImageDataset
from torch.utils.tensorboard import SummaryWriter
import Autoencoder
from torch_service import get_device, load_paths_from_folder, plot_classes_preds, show_torch_image

DATASET_PATH = "datasets/validation_test"

# Autoencoder and training data
autoencoder_path = "trained_autoencoders/run_13694de2d2424a379efaa48beb645d2dabcb4604.pt"
model = Autoencoder.AutoencoderVAEMediumConvSmallKernelBigBottleneck()
BATCH_SIZE = 1

# Tensorboard setup
writer = SummaryWriter("tests")
# log every n batches to Tensorboard
log_rhythm = 250

# load dataset from path
paths = load_paths_from_folder(DATASET_PATH)
transformed_dataset = GuiImageDataset.LazyLoadedGuiImageDataset(paths)
dataset_loader =  torch.utils.data.DataLoader(transformed_dataset, batch_size=BATCH_SIZE, num_workers=0)

print("Training Dataset:")
print(len(paths))
print(transformed_dataset.__getitem__(0).shape)

# obtain one batch of training images
dataiter = iter(dataset_loader)
images = dataiter.next()
# convert images to numpy for display
images = images.numpy()

# display an image
show_torch_image(images[0])

# init loss function
criterion = nn.MSELoss()

# push to available device
device = 'cpu'
print(device)
model.to(device)

# Load trained autoencoder
model.load_state_dict(torch.load(autoencoder_path, device))

# monitor training loss
test_loss = 0.0
running_loss = 0.0
i=0
print(len(dataset_loader))

# iterate over batches in dataset
for data in dataset_loader:
    print('Batch Index: {}'.format(i))
    i+=1
    images = data
    images = images.to(device)
    # predict
    outputs = model(images)
    # calculate loss
    loss = criterion(outputs, images)

    writer.add_scalar(
        'test loss',
        loss.item(),
        i
    )

    test_loss += loss.item()
    running_loss += loss.item()
    if i % log_rhythm == log_rhythm - 1:    # every n batches...
        print("Current Loss: {}".format(running_loss / log_rhythm))
        # ...log a figure showing the model's predictions
        writer.add_figure('predictions',
                        plot_classes_preds(model, images, outputs),
                        i)
        model.to(device)
        running_loss = 0.0
test_loss = test_loss/len(dataset_loader)
print('Overall Average Loss: {:.6f}'.format(test_loss))
