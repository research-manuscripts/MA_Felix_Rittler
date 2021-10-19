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
        super(Autoencoder2VAEMediumConv, self).__init__()

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
