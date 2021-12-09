import torch
import torch.nn as nn
import torch.nn.functional as F

"""A list of autoencoder architectures working with small images, e.g. 246x256 Pixel"""

"""
Autoencoder with small bottleneck working with 2 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVerySmallBottleneck(nn.Module):

    def __init__(self):
        super(AutoencoderVerySmallBottleneck, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.bottleneck = nn.Sequential(
            nn.Linear(32480, 800),
            nn.Linear(800, 60),
            nn.Linear(60, 800),
            nn.Linear(800, 32480)
        )

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(20, 4, 5, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(40, 20, 7, stride=1, padding=3, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # convolutional layer
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        size2 = out.size()
        out = F.relu(self.conv2(out))
        out, indices2 = self.maxpool2(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        # flatten
        out = out.view(out.size(0), -1)

        # fully connected layer
        out = torch.sigmoid(self.bottleneck(out))

        # reshape
        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out

"""
Autoencoder with very small bottleneck working with 2 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVerySmallBottleneck(nn.Module):

    def __init__(self):
        super(AutoencoderVerySmallBottleneck, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(4, stride=3, return_indices=True)

        self.bottleneck = nn.Sequential(
            nn.Linear(104160, 350),
            nn.Linear(350, 30),
            nn.Linear(30, 350),
            nn.Linear(350, 104160)
        )

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(4, stride=3)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(20, 4, 5, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )
        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(40, 20, 7, stride=1, padding=3, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # convolutional layer
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        size2 = out.size()
        out = F.relu(self.conv2(out))
        out, indices2 = self.maxpool2(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)

        # flatten
        out = out.view(out.size(0), -1)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck(out))

        # reshape
        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out


"""
Variational Autoencoder with small bottleneck working with 2 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAE(nn.Module):

    def __init__(self):
        super(AutoencoderVAE, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(32480, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            nn.Linear(60, 800),
            nn.Linear(800, 32480)
        )

        self.fc_mu = nn.Linear(60, 60)
        self.fc_logsigma = nn.Linear(60, 60)

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(20, 4, 5, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(40, 20, 7, stride=1, padding=3, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # convolutional layer
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        size2 = out.size()
        out = F.relu(self.conv2(out))
        out, indices2 = self.maxpool2(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)

        # flatten
        out = out.view(out.size(0), -1)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck2(out))

        # reshape
        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out

"""
Variational Autoencoder with big convolutional kernel working with 2 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAEBigConv(nn.Module):

    def __init__(self):
        super(AutoencoderVAEBigConv, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(4, 40, 32, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(32, stride=16, return_indices=True)
        self.conv2 = nn.Conv2d(40, 200, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(12, stride=6, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(11200, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            # Latent View
            nn.Linear(60, 800),
            nn.Linear(800, 11200)
        )

        self.fc_mu = nn.Linear(60, 60)
        self.fc_logsigma = nn.Linear(60, 60)

        self.unpool1 = nn.MaxUnpool2d(32, stride=16)
        self.unpool2 = nn.MaxUnpool2d(12, stride=6)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(40, 4, 32, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(200, 40, 4, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # convolutional layer
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        out = F.relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)

        # flatten
        out = out.view(out.size(0), -1)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck1(out))

        # variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck2(out))

        # reshape
        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out

"""
Variational Autoencoder with big convolutional kernel working with 2 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAEMediumConv(nn.Module):

    def __init__(self):
        super(AutoencoderVAEMediumConv, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(3, 20, 8, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(6, stride=6, return_indices=True)
        self.conv2 = nn.Conv2d(20, 50, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(4500, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            nn.Linear(60, 800),
            nn.Linear(800, 4500)
        )

        self.fc_mu = nn.Linear(60, 60)
        self.fc_logsigma = nn.Linear(60, 60)

        self.unpool1 = nn.MaxUnpool2d(6, stride=6)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(20, 3, 8, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(50, 20, 4, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # convolutional layer
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        out = F.relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)

        # flatten
        out = out.view(out.size(0), -1)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # fully-connected layer
        out = torch.sigmoid(self.bottleneck2(out))

        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out
