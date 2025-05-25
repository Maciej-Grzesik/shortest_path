import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt


def save(df, filename, title):
    df.to_csv(filename, index=False)
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

    plt.title(f"{title} - Execution Time by Vertices and Edges")
    plt.xlabel("Number of Vertices")
    plt.xticks(rotation=45)
    plt.ylabel("Execution Time (ms)")
    plt.legend(title="Edge Density")
    plt.tight_layout()

    plt.yscale("log")
    # plt.xscale("log")

    plt.savefig(filename, dpi=2000)
    
    
if __name__ == "__main__":
    df_bf = pd.read_csv("experiment/bellman_ford.csv")
    df_fw = pd.read_csv("experiment/floyd_warshall.csv")
    save(df_bf, "bellman_ford.png", "Bellman-Ford")
    save(df_fw, "floyd_warshall.png", "Floyd-Warshall")
