import torch
import Autoencoder
import GuiImageDataset
from torch_service import load_paths_from_folder
from torch import nn
import time
import torchvision.utils

# Autoencoder and training data
autoencoder_path = "trained_autoencoders/autoencoder_3547a5236c708c442558e4691d60e000893a122f.pt"
model = Autoencoder.AutoencoderVAEBigConvNoFully()

# load device and push to device
device = 'cpu'
print(device)
model.to(device)

# Load dataset
dataset = load_paths_from_folder("datasets/exp2")
transformed_dataset = GuiImageDataset.LazyLoadedGuiImageDataset(dataset)
dataset_loader =  torch.utils.data.DataLoader(transformed_dataset, batch_size=1, num_workers=0)

# Load trained autoencoder
model.load_state_dict(torch.load(autoencoder_path, device))
i=0
f = open("losses_real_jadx_a4.txt", "a")

for data in dataset_loader:
    images = data
    images = images.to(device)

    t_start = time.time()

    # predict
    pred = model(images)
    print("Prediction took {:6.3f}s".format(time.time()-t_start))

    loss = nn.MSELoss()(pred, images)
    print("Loss: ", loss.item())
    f.write("{}\n".format(loss.item()))
    print(pred.shape)

    # print original and prediction
    frame = images[0].detach()
    torchvision.utils.save_image(pred[0], "{}_pred_a4.png".format(i))
    torchvision.utils.save_image(frame.cpu(), "{}.png".format(i))
    i+=1
f.close()
