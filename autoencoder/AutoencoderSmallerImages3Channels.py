import torch
import torch.nn as nn
import torch.nn.functional as F

"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2VerySmallBottleneck(nn.Module):

    def __init__(self):
        super(Autoencoder2VerySmallBottleneck, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
            nn.Linear(32480, 800),
            nn.Linear(800, 60),
            nn.Linear(60, 800),
            nn.Linear(800, 32480)
        )

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)



        self.decoder1 = nn.Sequential(
            # nn.ConvTranspose2d(16, 16, 3, stride=2, padding=1, output_padding=1),
            # nn.ReLU(),
            # nn.BatchNorm2d(16),
            nn.ConvTranspose2d(20, 4, 5, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            # nn.ConvTranspose2d(16, 16, 3, stride=2, padding=1, output_padding=1),
            # nn.ReLU(),
            # nn.BatchNorm2d(16),
            nn.ConvTranspose2d(40, 20, 7, stride=1, padding=3, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # encoder
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        size2 = out.size()
        # print("Size after 1st encoder: ", out.size())
        out = F.relu(self.conv2(out))
        out, indices2 = self.maxpool2(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # print("Size after flatten:", out.size())
        out = torch.sigmoid(self.bottleneck1(out))
        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        # decoder
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())
        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)

        return out

"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2SmallStride(nn.Module):

    def __init__(self):
        super(Autoencoder2SmallStride, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(4, stride=3, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
            nn.Linear(104160, 350),
            nn.Linear(350, 30),
            nn.Linear(30, 350),
            nn.Linear(350, 104160)
        )

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(4, stride=3)



        self.decoder1 = nn.Sequential(
            # nn.ConvTranspose2d(16, 16, 3, stride=2, padding=1, output_padding=1),
            # nn.ReLU(),
            # nn.BatchNorm2d(16),
            nn.ConvTranspose2d(20, 4, 5, stride=1, padding=1, output_padding=0),
            nn.Sigmoid()
        )

        self.decoder2 = nn.Sequential(
            # nn.ConvTranspose2d(16, 16, 3, stride=2, padding=1, output_padding=1),
            # nn.ReLU(),
            # nn.BatchNorm2d(16),
            nn.ConvTranspose2d(40, 20, 7, stride=1, padding=3, output_padding=0),
            nn.Sigmoid()
        )

    def forward(self, x):
        # encoder
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        size2 = out.size()
        # print("Size after 1st encoder: ", out.size())
        out = F.relu(self.conv2(out))
        out, indices2 = self.maxpool2(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # print("Size after flatten:", out.size())
        out = torch.sigmoid(self.bottleneck1(out))
        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        # decoder
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())
        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)

        return out


"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2VAE(nn.Module):

    def __init__(self):
        super(Autoencoder2VAE, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
            nn.Linear(32480, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            # Latent View
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
        # encoder
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        size2 = out.size()
        # print("Size after 1st encoder: ", out.size())
        out = F.relu(self.conv2(out))
        out, indices2 = self.maxpool2(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # latent part 2
        out = torch.sigmoid(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # latent part 2
        out = torch.sigmoid(self.bottleneck2(out))


        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        # decoder
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())
        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)

        return out

"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2VAEBigConv(nn.Module):

    def __init__(self):
        super(Autoencoder2VAEBigConv, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(4, 40, 32, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(32, stride=16, return_indices=True)

        self.conv2 = nn.Conv2d(40, 200, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(12, stride=6, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
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
        # encoder
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        # print("Size before 2nd encoder: ", out.size())
        out = F.relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # latent part 2
        out = torch.sigmoid(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # latent part 2
        out = torch.sigmoid(self.bottleneck2(out))


        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        # decoder
        # print("Indices size: ", indices1.size())
        # print("Size after reshape: ", out.size())
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())

        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())
        # print("Size after decode2: ", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)

        return out

"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2VAEMediumConv(nn.Module):

    def __init__(self):
        super(Autoencoder2VAEMediumConv, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(3, 40, 16, stride=1, padding=16)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv2 = nn.Conv2d(40, 200, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv3 = nn.Conv2d(200, 400, 4, stride=1, padding=1)
        self.maxpool3 = nn.MaxPool2d(4, stride=2, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
            nn.Linear(10000, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            # Latent View
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
        # encoder
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        # print("Size before 2nd encoder: ", out.size())
        out = F.relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)
        out = F.relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # latent part 2
        out = torch.sigmoid(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # latent part 2
        out = torch.sigmoid(self.bottleneck2(out))


        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        out = self.unpool3(out, indices3, output_size=size3)
        # print("Size after unpool: ", out.size())

        out = self.decoder3(out)

        # decoder
        # print("Indices size: ", indices1.size())
        # print("Size after reshape: ", out.size())
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())

        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())
        # print("Size after decode2: ", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)

        return out

"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2VAEMediumConvBigKernel(nn.Module):

    def __init__(self):
        super(Autoencoder2VAEMediumConvBigKernel, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(3, 40, 32, stride=1, padding=16)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv2 = nn.Conv2d(40, 200, 4, stride=1, padding=1)
        self.maxpool2 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv3 = nn.Conv2d(200, 400, 4, stride=1, padding=1)
        self.maxpool3 = nn.MaxPool2d(4, stride=2, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
            nn.Linear(10000, 800),
            nn.Linear(800, 60)
        )

        self.bottleneck2 = nn.Sequential(
            # Latent View
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
        # encoder
        out = F.relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        # print("Size before 2nd encoder: ", out.size())
        out = F.relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)
        out = F.relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # latent part 2
        out = torch.sigmoid(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # latent part 2
        out = torch.sigmoid(self.bottleneck2(out))


        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        out = self.unpool3(out, indices3, output_size=size3)
        # print("Size after unpool: ", out.size())

        out = self.decoder3(out)

        # decoder
        # print("Indices size: ", indices1.size())
        # print("Size after reshape: ", out.size())
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())

        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())
        # print("Size after decode2: ", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)

        return out

"""
Auto-Encoder 4 - kann mit RGB-Bildern umgehen
Hinweis: Padding und Abschneiden der Dimensionen wurde verwendet, um die Dimensionen auf die richtige Größe zu bringen
"""
class Autoencoder2VAEMediumConvSmallKernel(nn.Module):

    def __init__(self):
        super(Autoencoder2VAEMediumConvSmallKernel, self).__init__()

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
            # Latent View
            nn.Linear(108800, 2000),
            nn.Linear(2000, 180)
        )

        self.bottleneck2 = nn.Sequential(
            # Latent View
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
        # encoder
        out = F.leaky_relu(self.conv1(x))
        size1 = out.size()
        out, indices1 = self.maxpool1(out)
        # print("Size before 2nd encoder: ", out.size())
        out = F.leaky_relu(self.conv2(out))
        size2 = out.size()
        out, indices2 = self.maxpool2(out)
        out = F.leaky_relu(self.conv3(out))
        size3 = out.size()
        out, indices3 = self.maxpool3(out)
        # print("Size after 2nd encoder and before flatten: ", out.size())

        originalC = out.size(1)
        originalH = out.size(2)
        originalW = out.size(3)
        out = out.view(out.size(0), -1)

        # latent part 2
        out = F.tanh(self.bottleneck1(out))

        # Variational part
        mu = self.fc_mu(out)
        logsigma = self.fc_logsigma(out)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        out = eps.mul(sigma).add_(mu)

        # latent part 2
        out = F.tanh(self.bottleneck2(out))


        # out = torch.reshape(out, (40,64,93))
        out = out.view(out.size(0), originalC, originalH, originalW)

        out = self.unpool3(out, indices3, output_size=size3)
        # print("Size after unpool: ", out.size())

        out = self.decoder3(out)

        # decoder
        # print("Indices size: ", indices1.size())
        # print("Size after reshape: ", out.size())
        # print("Size after reshape: ", out.size())
        out = self.unpool2(out, indices2, output_size=size2)
        # print("Size after unpool: ", out.size())

        out = self.decoder2(out)
        # print("Size after 2nd decoder:", out.size())
        # print("Size after decode2: ", out.size())

        out = self.unpool1(out, indices1, output_size=size1)
        # print("Size after 1st decoder:", out.size())

        out = self.decoder1(out)
        return out

class Autoencoder2VAEBigConvNoFully(nn.Module):

    def __init__(self):
        super(Autoencoder2VAEBigConvNoFully, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
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

        self.conv5 =  nn.Sequential(
            nn.Conv2d(1500, 5000, 7, stride=1, padding=3),
            nn.BatchNorm2d(5000),
        )
        self.maxpool5 = nn.MaxPool2d(4, stride=4, return_indices=True)

        self.fc_mu = nn.Linear(320, 320)
        self.fc_logsigma = nn.Linear(320, 320)

        self.unpool1 = nn.MaxUnpool2d(6, stride=6)
        self.unpool2 = nn.MaxUnpool2d(4, stride=4)
        self.unpool3 = nn.MaxUnpool2d(4, stride=4)
        self.unpool4 = nn.MaxUnpool2d(4, stride=4)
        self.unpool5 = nn.MaxUnpool2d(4, stride=4)

        self.decoder1 = nn.Sequential(
            nn.ConvTranspose2d(20, 3, 12, stride=1, padding=6, output_padding=0),
            nn.BatchNorm2d(3),
            nn.LeakyReLU()
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

        self.decoder5 = nn.Sequential(
            nn.ConvTranspose2d(5000, 1500, 7, stride=1, padding=3, output_padding=0),
            nn.BatchNorm2d(1500),
            nn.LeakyReLU()
        )

    def forward(self, x):
        # encoder
        x = F.leaky_relu(self.conv1(x))
        size1 = x.size()
        x, indices1 = self.maxpool1(x)
        # print("Size before 2nd encoder: ", x.size())
        x = F.leaky_relu(self.conv2(x))
        size2 = x.size()
        x, indices2 = self.maxpool2(x)
        x = F.leaky_relu(self.conv3(x))
        size3 = x.size()
        x, indices3 = self.maxpool3(x)
        x = F.leaky_relu(self.conv4(x))
        size4 = x.size()
        x, indices4 = self.maxpool4(x)
        #x = F.leaky_relu(self.conv5(x))
        #size5 = x.size()
        #x, indices5 = self.maxpool5(x)

        originalC = x.size(1)
        originalH = x.size(2)
        originalW = x.size(3)
        x = x.view(x.size(0), -1)

        # Variational part
        mu = self.fc_mu(x)
        logsigma = self.fc_logsigma(x)
        sigma = logsigma.exp()
        eps = torch.randn_like(sigma)
        x = eps.mul(sigma).add_(mu)
        x = x.view(x.size(0), originalC, originalH, originalW)

        #x = self.unpool5(x, indices5, output_size=size5)
        #x = self.decoder5(x)

        x = self.unpool4(x, indices4, output_size=size4)
        x = self.decoder4(x)
        x = self.unpool3(x, indices3, output_size=size3)
        x = self.decoder3(x)
        x = self.unpool2(x, indices2, output_size=size2)
        x = self.decoder2(x)
        x = self.unpool1(x, indices1, output_size=size1)
        x = self.decoder1(x)
        return x
