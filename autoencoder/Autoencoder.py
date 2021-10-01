import torch
import torch.nn as nn
import torch.nn.functional as F


class Autoencoder1(nn.Module):
    def __init__(self):
        super(Autoencoder1, self).__init__()
        self.encoder = nn.Sequential(
            # nn.Linear(28 * 28, 128),
            nn.Linear(1500, 128),
            nn.ReLU(True),
            nn.Linear(128, 64),
            nn.ReLU(True),
            nn.Linear(64, 12),
            nn.ReLU(True),
            nn.Linear(12, 3)
        )
        self.decoder = nn.Sequential(
            nn.Linear(3, 12),
            nn.ReLU(True),
            nn.Linear(12, 64),
            nn.ReLU(True),
            nn.Linear(64, 128),
            nn.ReLU(True),
            nn.Linear(128, 1500),
            nn.Tanh()
        )

    def forward(self, x):
        x = self.encoder(x)
        x = self.decoder(x)
        return x

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
            nn.Linear(58880, 400),
            nn.Linear(400, 30),
            nn.Linear(30, 400),
            nn.Linear(400, 58880)
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
class Autoencoder2BiggerBottleNeck(nn.Module):

    def __init__(self):
        super(Autoencoder2BiggerBottleNeck, self).__init__()

        self.size1 = 0
        self.size2 = 0

        # input: batch x 3 x 32 x 32 -> output: batch x 16 x 16 x 16
        self.conv1 = nn.Conv2d(4, 20, 5, stride=1, padding=1)
        self.maxpool1 = nn.MaxPool2d(8, stride=8, return_indices=True)

        self.conv2 = nn.Conv2d(20, 40, 7, stride=1, padding=3)
        self.maxpool2 = nn.MaxPool2d(6, stride=6, return_indices=True)

        self.bottleneck1 = nn.Sequential(
            # Latent View
            nn.Linear(26040, 1000),
            nn.Linear(1000, 300),
            nn.Linear(300, 1000),
            nn.Linear(1000, 26040)
        )

        self.unpool1 = nn.MaxUnpool2d(8, stride=8)
        self.unpool2 = nn.MaxUnpool2d(6, stride=6)



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
        # print("Size after 1st encoder: ", out.size())
        out = F.relu(self.conv2(out))
        size2 = out.size()
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