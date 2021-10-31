# Autoencoder

This repository contains all code regarding the autoencoders to encode GUI information.

# Prerequisites

1. Install [Python 3](https://www.python.org/downloads/). I recommend the usage of [virtual environments](https://docs.python.org/3/library/venv.html).
2. Install all requirements via `pip install -r requirements.txt`. The used PyTorch version is `1.9.1+cu111`. 
Feel free to replace it with the version that [fits for you](https://pytorch.org/get-started/locally/)
3. Generate your own dataset [here](https://github.com/research-manuscripts/MA_Felix_Rittler/tree/main/jadx_mock)

# Usage
- To train you own autoencoder edit `train_autoencoder.py` by replacing the used dataset, autoencoder architecture or hyperparameters and run it by executing 
`python train_autoencoder.py`. The trained autoencoder will be saved into the file `trained_autoencoder.pt`
- If you want to inspect a autoencoder architecture edit `check_autoencoder.py` by replacing the used dataset, autoencoder architecture or trained autoencoder. It will print the first 
two images from the dataset and the reconstructions by the selected autoencoder.
- The implemented autoencoder architectures can be found in `Autoencoder.py`. Please note that the architectures require a image size of 935x900 Pixel.
- The trained autoencoders can be found in the directory `trained_autoencoders` (TODO)
