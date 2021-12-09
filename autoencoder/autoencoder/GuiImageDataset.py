from torchvision import transforms
from torch.utils.data import Dataset
from PIL import Image

"""
Dataset of images lazy loaded during access
"""
class LazyLoadedGuiImageDataset(Dataset):
    def __init__(self, paths):
        """
        Init the dataset
        :param list(str) paths: A list of paths containing all images for the dataset
        """
        self.paths = paths

    def __len__(self):
        """
        Denotes the total number of samples
        """
        return len(self.paths)

    def __getitem__(self, index):
        """
        Generates one sample of data and converts to RGB
        """
        # Select sample
        x = Image.open(self.paths[index]).convert('RGB')  # load lazily
        x = self.transform(x)
        return x

    transform = transforms.Compose([
        transforms.ToTensor()]
    )

"""
Dataset of images resized to 246x256 Pixel lazy loaded during access
"""
class LazyLoadedSmallGuiImageDataset(Dataset):
    def __init__(self, paths):
        """
        Init the dataset
        :param list(str) paths: A list of paths containing all images for the dataset
        """
        self.paths = paths

    def __len__(self):
        'Denotes the total number of samples'
        return len(self.paths)

    def __getitem__(self, index):
        """
        Generates one sample of data and converts to RGB
        """
        # Select sample
        x = Image.open(self.paths[index]).convert('RGB')  # load lazily
        x = self.transform(x)
        return x

    transform = transforms.Compose([
        transforms.Resize((246, 256)),
        transforms.ToTensor(),
        ]
    )
