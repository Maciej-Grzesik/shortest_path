import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

df = pd.read_csv("../experiment/bellman_ford.csv")
# df = pd.read_csv("../experiment/floyd_warshall.csv")

df['Vertices'] = df['Vertices'].astype(str)
df['Edges'] = df['Edges'].astype(str)

plt.figure(figsize=(20, 10))
sns.violinplot(
    data=df,
    x="Vertices",
    y="Time (ms)",
    hue="Edges",
    split=False,
    scale="width",
    inner="quartile"
)
sns.pointplot(
    data=df,
    x="Vertices",
    y="Time (ms)",
    hue="Edges",
    dodge=0.5,
    join=True,
    markers="D",
    linestyles="--",
    ci="sd",
    palette="dark"
)

plt.title("Violin Plot - Execution Time by Vertices and Edges")
plt.xlabel("Number of Vertices")
plt.xticks(rotation=45)
plt.ylabel("Execution Time (ms)")
plt.legend(title="Edge Density")
plt.tight_layout()

plt.savefig("bellman_ford_violin_plot.png", dpi=2000)
