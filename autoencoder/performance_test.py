import torch
import Autoencoder
import GuiImageDataset
from torch_service import load_paths_from_folder
from torch import nn
import itertools
import time
import torchvision.utils
import numpy as np

# Autoencoder and training data
autoencoder_paths = [
    "trained_autoencoders/a1.pt", "trained_autoencoders/a2.pt",
    "trained_autoencoders/a3.pt", "trained_autoencoders/a4.pt"
]
trained_models = [
    (Autoencoder.AutoencoderVAEMediumConvBigKernel(), "run_3b7d5453ce41baeba6fcab6937df2c16a4fc9523.pt"),
    (Autoencoder.AutoencoderVAEBigConvNoFully(), "run_8082f9d131644f4a2d6ea92f3a0858599d766f1a.pt"),
    (Autoencoder.AutoencoderVAEMediumConvSmallKernelBigBottleneck(), "run_c06146c6b89dddb2781126fa3e8ba7d36693f78b.pt"),
    (Autoencoder.AutoencoderVAEMediumConvSmallKernel(), "run_012f2d5ce7ab9b1e437ffe10fc120ec1107fb3a6.pt")
]
dummy_input = torch.randn(1, 3, 935, 900, dtype=torch.float)
torch.save(dummy_input, 'performance_test_tensor.pt')

# load device and push to device
device = 'cuda:0'
dummy_input = dummy_input.to(device)
print(device)

for (model, autoencoder_path) in trained_models:
    print(autoencoder_path)
    # Load trained autoencoder
    model.load_state_dict(torch.load(autoencoder_path, device))
    model.to(device)
    dummy_input = dummy_input.to(device)

    f = open("performance_test_results.txt", "a")
    starter, ender = torch.cuda.Event(enable_timing=True), torch.cuda.Event(enable_timing=True)
    repetitions = 10
    timings_gpu = np.zeros((repetitions,1))

    #GPU-WARM-UP
    for _ in range(10):
        _ = model(dummy_input)

    print("Warm-Up Complete")
    # MEASURE PERFORMANCE
    with torch.no_grad():
        for rep in range(repetitions):
            starter.record()
            _ = model(dummy_input)
            ender.record()
            # WAIT FOR GPU SYNC
            torch.cuda.synchronize()
            curr_time = starter.elapsed_time(ender)
            print(curr_time)
            timings_gpu[rep] = curr_time
    mean_syn = np.sum(timings_gpu) / repetitions
    std_syn = np.std(timings_gpu)
    print(mean_syn)
    f.write("Autoencoder {} on {}: Mean: {}, Std: {}\n".format(autoencoder_path, device, mean_syn, std_syn))

    device = 'cpu'
    dummy_input = dummy_input.to(device)
    model.to(device)

    timings_cpu = np.zeros((repetitions,1))
    with torch.no_grad():
        for rep in range(repetitions):
            startTime = time.time()
            _ = model(dummy_input)
            executionTime = (time.time() - startTime)
            timings_cpu[rep] = executionTime

    mean_syn = np.sum(timings_cpu) / repetitions
    std_syn = np.std(timings_cpu)
    print(mean_syn)
    f.write("Autoencoder {} on {}: Mean: {}, Std: {}\n".format(autoencoder_path, device, mean_syn, std_syn))
    f.close()
