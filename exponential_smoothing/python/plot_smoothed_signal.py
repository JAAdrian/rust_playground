import re

import numpy
from matplotlib import pyplot

NOISY_SIGNAL_FILEPATH = r"data/noisy_input_fs1000Hz.txt"
NAIVLY_SMOOTHED_SIGNAL_FILEPATH = r"data/naivly_smoothed_output_fs1000Hz.txt"
ALPHA_BETA_SMOOTHED_SIGNAL_FILEPATH = r"data/alpha_beta_smoothed_output_fs1000Hz.txt"

SAMPLE_RATE_EXPRESSION = r".*_fs(\d+)Hz"

if __name__ == "__main__":
    noisy_data = numpy.fromfile(NOISY_SIGNAL_FILEPATH, sep="\n")
    naivly_smoothed_data = numpy.fromfile(NAIVLY_SMOOTHED_SIGNAL_FILEPATH, sep="\n")
    alpha_beta_smoothed_data = numpy.fromfile(
        ALPHA_BETA_SMOOTHED_SIGNAL_FILEPATH, sep="\n"
    )
    len_data = len(naivly_smoothed_data)

    sample_rate_match = re.search(
        SAMPLE_RATE_EXPRESSION, NAIVLY_SMOOTHED_SIGNAL_FILEPATH
    )
    if sample_rate_match:
        sample_rate = int(sample_rate_match.groups()[0])

    time = numpy.arange(len_data) / sample_rate

    fig, ax = pyplot.subplots(figsize=(12, 12 / 1.618))
    ax.plot(time, noisy_data, label="Raw")
    ax.plot(time, naivly_smoothed_data, label="Exponentially Smoothed", linewidth=3)
    ax.plot(time, alpha_beta_smoothed_data, label=r"$\alpha-\beta$ Smoothed", linewidth=3)
    ax.legend()

    pyplot.show()
