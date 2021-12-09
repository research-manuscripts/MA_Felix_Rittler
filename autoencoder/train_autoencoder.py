import torch
import numpy as np
from torch import nn
import GuiImageDataset
from torch.utils.tensorboard import SummaryWriter
import Autoencoder
from torch_service import get_device, load_paths_from_folder, plot_classes_preds, show_torch_image

# PLEASE CONFIGURE THESE PARAMETERS BEFORE RUNNING
DATASET_PATH = "datasets/train_dataset"
model = Autoencoder.Autoencoder4()
BATCH_SIZE = 16
LEARNING_RATE = 2e-3
NUMBER_OF_EPOCHS = 3
# END OF PARAMETERS TO CONFIGURE

# Tensorboard setup
writer = SummaryWriter()
# log every n batches to Tensorboard
log_rhythm = 25

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

# display some images
for idx in np.arange(2):
    show_torch_image(images[idx])

# init criterion and optimizer
criterion = nn.BCELoss()
optimizer = torch.optim.Adamax(
    model.parameters(), lr=LEARNING_RATE
)

# push to available device
device = get_device()
print(device)
model.to(device)

torch.autograd.set_detect_anomaly(True)

# training
for epoch in range(1, NUMBER_OF_EPOCHS+1):
    # monitor training loss
    train_loss = 0.0
    running_loss = 0.0
    i=0
    print(len(dataset_loader))

    # iterate over batches in dataset
    for data in dataset_loader:
        print('Batch Index: {}'.format(i))
        i+=1

        images = data
        images = images.to(device)
        optimizer.zero_grad()
        # predict
        outputs = model(images)
        # calculate loss
        loss = criterion(outputs, images)
        # backwards pass
        loss.backward()

        torch.nn.utils.clip_grad_norm_(model.parameters(), 1)

        optimizer.step()
        train_loss += loss.item()
        running_loss += loss.item()

        if i % log_rhythm == log_rhythm - 1:    # every n batches...
            # ...log the running loss
            writer.add_scalar('training loss',
                            running_loss / log_rhythm,
                            i)

            print("Current Loss: {}".format(train_loss/i))

            # ...log a figure showing the model's predictions
            writer.add_figure('predictions',
                            plot_classes_preds(model, images, outputs),
                            i)
            model.to(device)
            running_loss = 0.0

    train_loss = train_loss/len(dataset_loader)
    print('Epoch: {} \tTraining Loss: {:.6f}'.format(epoch, train_loss))

# save trained autoencoder
torch.save(model.state_dict(), "trained_autoencoders/trained_autoencoder.pt")
