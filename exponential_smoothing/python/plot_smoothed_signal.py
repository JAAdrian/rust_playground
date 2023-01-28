import re

import numpy
from matplotlib import pyplot

NOISY_SIGNAL_FILEPATH = r"data/noisy_input_fs1000Hz.txt"
SMOOTHED_SIGNAL_FILEPATH = r"data/smoothed_output_fs1000Hz.txt"

SAMPLE_RATE_EXPRESSION = r".*_fs(\d+)Hz"

if __name__ == '__main__':
    noisy_data = numpy.fromfile(NOISY_SIGNAL_FILEPATH, sep="\n")
    smoothed_data = numpy.fromfile(SMOOTHED_SIGNAL_FILEPATH, sep="\n")
    len_data = len(smoothed_data)

    sample_rate_match = re.search(SAMPLE_RATE_EXPRESSION, SMOOTHED_SIGNAL_FILEPATH)
    if sample_rate_match:
        sample_rate = int(sample_rate_match.groups()[0])

    time = numpy.arange(len_data) / sample_rate

    fig, ax = pyplot.subplots(figsize=(12, 12/1.618))
    ax.plot(time, noisy_data, label="Raw")
    ax.plot(time, smoothed_data, label="Smoothed", linewidth=3)
    ax.legend()

    pyplot.show()
