import torch
import torch.nn as nn
import torch.nn.functional as F

"""A list of autoencoder architectures working with normal sized images, e.g. 935x900 Pixel and 3 Input channels (RGB images, no alpha channel)"""

"""
Variational Autoencoder working with 3 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAEMediumConv(nn.Module):

    def __init__(self):
        super(AutoencoderVAEMediumConv, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(3, 40, 16, stride=1, padding=16)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv2 = nn.Conv2d(40, 200, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv3 = nn.Conv2d(200, 400, 4, stride=1, padding=1)
        self.maxpool3 = nn.MaxPool2d(4, stride=2, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(10000, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            nn.Linear(60, 800),
            nn.Linear(800, 10000)
        )

        self.fc_mu = nn.Linear(60, 60)
        self.fc_logsigma = nn.Linear(60, 60)

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(8, stride=8)
        self.unpool3 = nn.MaxUnpool2d(4, stride=2)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(40, 3, 16, stride=1, padding=16, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(200, 40, 4, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder3 = nn.Sequential(
            nn.ConvTranspose2d(400, 200, 4, stride=1, padding=1, output_padding=0),
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
        out = F.relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)

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
        out = self.unpool3(out, indices3, output_size=size3)
        out = self.decoder3(out)
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out

"""
Variational Autoencoder with big convolutional kernel working with 3 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAEMediumConvBigKernel(nn.Module):

    def __init__(self):
        super(AutoencoderVAEMediumConvBigKernel, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(3, 40, 32, stride=1, padding=16)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv2 = nn.Conv2d(40, 200, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(8, stride=8, return_indices=True)
        self.conv3 = nn.Conv2d(200, 400, 4, stride=1, padding=1)
        self.maxpool3 = nn.MaxPool2d(4, stride=2, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(10000, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            nn.Linear(60, 800),
            nn.Linear(800, 10000)
        )

        self.fc_mu = nn.Linear(60, 60)
        self.fc_logsigma = nn.Linear(60, 60)

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(8, stride=8)
        self.unpool3 = nn.MaxUnpool2d(4, stride=2)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(40, 3, 32, stride=1, padding=16, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(200, 40, 4, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder3 = nn.Sequential(
            nn.ConvTranspose2d(400, 200, 4, stride=1, padding=1, output_padding=0),
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
        out = F.relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)

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
        out = self.unpool3(out, indices3, output_size=size3)
        out = self.decoder3(out)
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out

"""
Variational Autoencoder with medium sized convolutional kernel working with 3 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAEMediumConvSmallKernel(nn.Module):

    def __init__(self):
        super(AutoencoderVAEMediumConvSmallKernel, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(3, 40, 16, stride=1, padding=16)
        self.maxpool1 = nn.MaxPool2d(6, stride=6, return_indices=True)

        self.conv2 = nn.Conv2d(40, 200, 10, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.conv3 = nn.Conv2d(200, 400, 4, stride=1, padding=1)
        self.maxpool3 = nn.MaxPool2d(4, stride=2, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(108800, 2000),
            nn.Linear(2000, 180)
        )

        self.bottleneck2 = nn.Sequential(
            nn.Linear(180, 2000),
            nn.Linear(2000, 108800)
        )

        self.fc_mu = nn.Linear(180, 180)
        self.fc_logsigma = nn.Linear(180, 180)

        self.unpool1 = nn.MaxUnpool2d(6, stride=6)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)
        self.unpool3 = nn.MaxUnpool2d(4, stride=2)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(40, 3, 16, stride=1, padding=16, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(200, 40, 10, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder3 = nn.Sequential(
            nn.ConvTranspose2d(400, 200, 4, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # convolutional layer
        out = F.leaky_relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        out = F.leaky_relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)
        out = F.leaky_relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)

        # flatten
        out = out.view(out.size(0), -1)

        # fully-connected layer
        out = F.tanh(self.bottleneck1(out))

        # variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # fully-connected layer
        out = F.tanh(self.bottleneck2(out))

        # reshape
        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool3(out, indices3, output_size=size3)
        out = self.decoder3(out)
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out

"""
Variational Autoencoder with working with 4 Convolutional and no (!) Fully-connected layer
"""
class AutoencoderVAEBigConvNoFully(nn.Module):

    def __init__(self):
        super(AutoencoderVAEBigConvNoFully, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Sequential(
            nn.Conv2d(3, 20, 12, stride=1, padding=6),
            nn.BatchNorm2d(20),
        )
        self.maxpool1 = nn.MaxPool2d(6, stride=6, return_indices=True)

        self.conv2 =  nn.Sequential(
            nn.Conv2d(20, 40, 8, stride=1, padding=4),
            nn.BatchNorm2d(40),
        )
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.conv3 =  nn.Sequential(
            nn.Conv2d(40, 60, 7, stride=1, padding=3),
            nn.BatchNorm2d(60),
        )
        self.maxpool3 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.conv4 =  nn.Sequential(
            nn.Conv2d(60, 80, 7, stride=1, padding=3),
            nn.BatchNorm2d(80),
        )
        self.maxpool4 = nn.MaxPool2d(4, stride=4, return_indices=True)


        self.fc_mu = nn.Linear(320, 320)
        self.fc_logsigma = nn.Linear(320, 320)

        self.unpool1 = nn.MaxUnpool2d(6, stride=6)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)
        self.unpool3 = nn.MaxUnpool2d(4, stride=4)
        self.unpool4 = nn.MaxUnpool2d(4, stride=4)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(20, 3, 12, stride=1, padding=6, output_padding=0),
            nn.BatchNorm2d(3),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(40, 20, 8, stride=1, padding=4, output_padding=0),
            nn.BatchNorm2d(20),
            nn.LeakyReLU()
        )

        self.decoder3 = nn.Sequential(
            nn.ConvTranspose2d(60, 40, 7, stride=1, padding=3, output_padding=0),
            nn.BatchNorm2d(40),
            nn.LeakyReLU()
        )

        self.decoder4 = nn.Sequential(
            nn.ConvTranspose2d(80, 60, 7, stride=1, padding=3, output_padding=0),
            nn.BatchNorm2d(60),
            nn.LeakyReLU()
        )

    def forward(self, x):
        # convolutional layer
        x = F.leaky_relu(self.conv1(x))
        size1 = x.size()
        x, indices1 = self.maxpool1(x)
        x = F.leaky_relu(self.conv2(x))
        size2 = x.size()
        x, indices2 = self.maxpool2(x)
        x = F.leaky_relu(self.conv3(x))
        size3 = x.size()
        x, indices3 = self.maxpool3(x)
        x = F.leaky_relu(self.conv4(x))
        size4 = x.size()
        x, indices4 = self.maxpool4(x)

        originalC = x.size(1)
        originalH = x.size(2)
        originalW = x.size(3)

        # flatten
        x = x.view(x.size(0), -1)

        # variational part
        mu = self.fc_mu(x)
        logsigma = self.fc_logsigma(x)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        x = eps.mul(sigma).add_(mu)

        # reshape
        x = x.view(x.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        x = self.unpool4(x, indices4, output_size=size4)
        x = self.decoder4(x)
        x = self.unpool3(x, indices3, output_size=size3)
        x = self.decoder3(x)
        x = self.unpool2(x, indices2, output_size=size2)
        x = self.decoder2(x)
        x = self.unpool1(x, indices1, output_size=size1)
        x = self.decoder1(x)

        return x

"""
Variational Autoencoder with big bottleneck working with 3 Convolutional and 4 Fully-connected layer
"""
class AutoencoderVAEMediumConvSmallKernelBigBottleneck(nn.Module):

    def __init__(self):
        super(AutoencoderVAEMediumConvSmallKernelBigBottleneck, self).__init__()

        self.size1 = 0
        self.size2 = 0

        self.conv1 = nn.Conv2d(3, 40, 16, stride=1, padding=16)
        self.maxpool1 = nn.MaxPool2d(6, stride=6, return_indices=True)

        self.conv2 = nn.Conv2d(40, 200, 10, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.conv3 = nn.Conv2d(200, 400, 4, stride=1, padding=1)
        self.maxpool3 = nn.MaxPool2d(4, stride=2, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            nn.Linear(108800, 4000),
            nn.Linear(4000, 360)
        )

        self.bottleneck2 = nn.Sequential(
            nn.Linear(360, 4000),
            nn.Linear(4000, 108800)
        )

        self.fc_mu = nn.Linear(360, 360)
        self.fc_logsigma = nn.Linear(360, 360)

        self.unpool1 = nn.MaxUnpool2d(6, stride=6)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)
        self.unpool3 = nn.MaxUnpool2d(4, stride=2)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(40, 3, 16, stride=1, padding=16, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            nn.ConvTranspose2d(200, 40, 10, stride=1, padding=1, output_padding=0),
            nn.LeakyReLU()
        )

        self.decoder3 = nn.Sequential(
            nn.ConvTranspose2d(400, 200, 4, stride=1, padding=1, output_padding=0),
            nn.LeakyReLU()
        )

    def forward(self, x):
        # convolutional layer
        out = F.leaky_relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        out = F.leaky_relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)
        out = F.leaky_relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)

        # flatten
        out = out.view(out.size(0), -1)

        # fully-connected layer
        out = torch.tanh(self.bottleneck1(out))

        # variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # fully-connected layer
        out = torch.tanh(self.bottleneck2(out))

        # reshape
        out = out.view(out.size(0), originalC, originalH, originalW)

        # deconvolutional layer
        out = self.unpool3(out, indices3, output_size=size3)
        out = self.decoder3(out)
        out = self.unpool2(out, indices2, output_size=size2)
        out = self.decoder2(out)
        out = self.unpool1(out, indices1, output_size=size1)
        out = self.decoder1(out)

        return out
