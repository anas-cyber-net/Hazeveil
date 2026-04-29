import json

import numpy as np

import matplotlib.pyplot as plt

from sklearn.ensemble import RandomForestClassifier

def load_session(filename):

    with open(filename) as f:

        return json.load(f)

def extract_features(events):

    downs = [e for e in events if e['type'] == 'down']

    if len(downs) < 2:

        return None

    delays = []

    for i in range(1, len(downs)):

        delay = downs[i]['time'] - downs[i-1]['time']

        if 0 < delay < 2.0:

            delays.append(delay)

    if len(delays) < 10:

        return None

    return [

        np.mean(delays),

        np.std(delays),

        np.min(delays),

        np.max(delays),

        np.percentile(delays, 25),

        np.percentile(delays, 75),

        np.median(delays),

    ]

baseline_X = []

hazeveil_X = []

for i in range(1, 11):

    f = extract_features(load_session(f'baseline_{i}.json'))

    if f:

        baseline_X.append(f)

    f = extract_features(load_session(f'hazeveil_{i}.json'))

    if f:

        hazeveil_X.append(f)

baseline_X = np.array(baseline_X)

hazeveil_X = np.array(hazeveil_X)

train_X = baseline_X[:8]

test_baseline = baseline_X[8:]

test_hazeveil = hazeveil_X[8:]

clf = RandomForestClassifier(n_estimators=100, random_state=42)

clf.fit(train_X, np.ones(len(train_X)))

baseline_score = clf.score(test_baseline, np.ones(len(test_baseline)))

hazeveil_preds = clf.predict(test_hazeveil)

hazeveil_score = 1 - np.mean(hazeveil_preds)

print("=" * 40)

print("Benchmark Results")

print("=" * 40)

print(f"Recognition without HazeVeil: {baseline_score:.1%}")

print(f"Recognition with HazeVeil:    {hazeveil_score:.1%}")

print(f"Drop: {(baseline_score - hazeveil_score):.1%}")

print("=" * 40)

labels = ['Without HazeVeil', 'With HazeVeil']

values = [baseline_score * 100, hazeveil_score * 100]

colors = ['#ff4444', '#44aa44']

plt.figure(figsize=(8, 5))

bars = plt.bar(labels, values, color=colors, width=0.4)

plt.title('Behavioral Biometrics Recognition Rate', fontsize=14)

plt.ylabel('Recognition Rate (%)')

plt.ylim(0, 100)

for bar, val in zip(bars, values):

    plt.text(bar.get_x() + bar.get_width()/2,

             bar.get_height() + 1,

             f'{val:.1f}%',

             ha='center', fontsize=12, fontweight='bold')

plt.tight_layout()

plt.savefig('benchmark_results.png', dpi=150)

print("Saved: benchmark_results.png")
