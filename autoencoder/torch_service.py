import numpy as np
import torch
import matplotlib.pyplot as plt
import torch.nn.functional as F

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

# helper function to show an image
# (used in the `plot_classes_preds` function below)
def matplotlib_imshow(img, one_channel=False):
    if one_channel:
        img = img.mean(dim=0)
    npimg = img.cpu().detach().numpy()
    if one_channel:
        plt.imshow(npimg, cmap="Greys")
    else:
        plt.imshow(np.transpose(npimg, (1, 2, 0)))

# Tensorboard helper functions
def images_to_probs(net, images):
    '''
    Generates predictions and corresponding probabilities from a trained
    network and a list of images
    '''
    output = net(images)
    # convert output probabilities to predicted class
    _, preds_tensor = torch.max(output, 1)
    preds = np.squeeze(preds_tensor.cpu().numpy())
    return preds, [F.softmax(el, dim=0)[i].item() for i, el in zip(preds, output)]


def plot_classes_preds(net, images, outputs):
    '''
    Generates matplotlib Figure using a trained network, along with images
    and labels from a batch, that shows the network's top prediction along
    with its probability, alongside the actual label, coloring this
    information based on whether the prediction was correct or not.
    Uses the "images_to_probs" function.
    '''
    net = net.cpu()
    # preds, probs = images_to_probs(net, images)
    # plot the images in the batch, along with predicted and true labels
    fig = plt.figure(figsize=(12, 48))
    for idx in np.arange(images.size(0)):
        # ax = fig.add_subplot(1, 4, idx+1, xticks=[], yticks=[])
        # matplotlib_imshow(images[idx], one_channel=False)
        matplotlib_imshow(outputs[idx], one_channel=False)

        # ax.set_title("idx: {0}".format(loss))
        # ax.set_title("{0}, {1:.1f}%\n(label: {2})".format(
        #     classes[preds[idx]],
        #     probs[idx] * 100.0,
        #     classes[outputs[idx]]),
        #             color=("green" if preds[idx]==outputs[idx].item() else "red"))
    return fig