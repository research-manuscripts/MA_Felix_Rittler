import torch
import numpy as np
from torch import nn
import GuiImageDataset
from torch.utils.tensorboard import SummaryWriter
import Autoencoder
from torch_service import get_device, load_paths_from_folder, plot_classes_preds, show_torch_image

DATASET_PATH = "datasets/validation_set"

# Autoencoder and training data
autoencoder_path = "run_c06146c6b89dddb2781126fa3e8ba7d36693f78b.pt"
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
device = get_device()
print(device)
model.to(device)

# Load trained autoencoder
model.load_state_dict(torch.load(autoencoder_path, device))

# monitor training loss
test_loss = 0.0
i=0
print(len(dataset_loader))

# iterate over batches in dataset
for data in dataset_loader:
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
    if (i % 25 == 0):
        print('Batch Index: {}'.format(i))
        print("Current Loss: {}".format(loss.item()))
    i+=1
test_loss = test_loss/len(dataset_loader)
print('Overall Average Loss: {:.6f}'.format(test_loss))
writer.add_scalar(
    'overall_loss',
    test_loss,
    0
)