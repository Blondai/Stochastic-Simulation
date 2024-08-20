import matplotlib.pyplot as plt
import numpy as np

with open('test.txt', 'r') as file:
    data: list[str] = file.readlines()
numbers: list[float] = []
for datum in data:
    numbers.append(float(datum))


def normal_density(mu: float, sigma: float, input_value: float | np.array):
    return 1 / np.sqrt(2 * np.pi * sigma ** 2) * np.exp(- (input_value - mu) ** 2 / (2 * sigma ** 2))
x_values: np.array = np.linspace(-4, 4, 1000)
y_values: np.array = normal_density(0, 1, x_values)

fig, ax = plt.subplots()
ax.hist(numbers, bins=1000, density=True)
ax.plot(x_values, y_values)
# plt.savefig("example.png", dpi=1000)

plt.show()
