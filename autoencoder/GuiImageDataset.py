from torchvision import transforms
from torch.utils.data import Dataset
from PIL import Image

class GuiImageDataset(Dataset):
    def __init__(self, X):
        'Initialization'
        self.X = X

    def __len__(self):
        'Denotes the total number of samples'
        return len(self.X)

    def __getitem__(self, index):
        'Generates one sample of data'
        # Select sample
        image = self.X[index]
        X = self.transform(image)
        return X

    transform = transforms.Compose([
        transforms.ToPILImage(),
        transforms.ToTensor()])

class LazyLoadedGuiImageDataset(Dataset):
    def __init__(self, paths):
        'Initialization'
        self.paths = paths

    def __len__(self):
        'Denotes the total number of samples'
        return len(self.paths)

    def __getitem__(self, index):
        'Generates one sample of data'
        # Select sample
        x = Image.open(self.paths[index])  # load lazily
        x = self.transform(x)
        return x

    transform = transforms.Compose([
        # transforms.ToPILImage(),
        transforms.ToTensor()])