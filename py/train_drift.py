import pandas as pd
from sklearn.ensemble import IsolationForest
import joblib, sys

df = pd.read_csv("output/sessions.csv")
print("Columns in CSV:", df.columns.to_list())
features = df[["window_switches", "distractor_hits", "duration_min"]]

model = IsolationForest(contamination=0.1, random_state=42)
model.fit(features)

joblib.dump(model, "py/drift_model.joblib")
print("Model trained on ", len(df),"sessions")