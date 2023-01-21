import pathlib

import numpy
from matplotlib import pyplot
from scipy import signal

DO_PLOT_SIGNALS = False

OUTPUT_FILENAME = r"data/noisy_input.txt"

SIGNAL_LEN_SEC = 2

SAMPLE_RATE = 1000
FREQUENCY = 4
NOISE_STD = 0.1


if __name__ == "__main__":
    output_filename_raw = pathlib.Path(OUTPUT_FILENAME)
    output_filename = pathlib.Path(output_filename_raw.parent, output_filename_raw.stem + f"_fs{SAMPLE_RATE}Hz").with_suffix(".txt")

    if not output_filename.parent.is_dir():
        output_filename.parent.mkdir()

    signal_len_samples = round(SAMPLE_RATE * SIGNAL_LEN_SEC)

    time = numpy.arange(signal_len_samples) / SAMPLE_RATE
    sawtooth = signal.sawtooth(2 * numpy.pi * FREQUENCY * time, width=0.5)

    noise = NOISE_STD * numpy.random.randn(signal_len_samples)

    noisy_signal = sawtooth + noise

    if DO_PLOT_SIGNALS:
        pyplot.plot(time, noisy_signal)
        pyplot.show()

    numpy.savetxt(output_filename, noisy_signal, delimiter="\n")
