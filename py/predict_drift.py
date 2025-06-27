import pandas as pd, joblib, sys

model = joblib.load("py/drift_model.joblib")
df = pd.read_csv(sys.argv[1])
features = df[["window_switches", "distractor_hits", "duration_min"]]
score = model.decision_function(features)[0]
label = model.predict(features)[0]

print(f"{label},{score}")