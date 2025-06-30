# This files contains your custom actions which can be used to run
# custom Python code.
#
# See this guide on how to implement these action:
# https://rasa.com/docs/rasa/custom-actions


# This is a simple example for a custom action which utters "Hello World!"
import requests
from rasa_sdk import Action, Tracker
from rasa_sdk.executor import CollectingDispatcher
from typing import Any, Text, Dict, List

RUST_API = "http://localhost:9090"

class ActionStartFocus(Action):
    def name(self):
        return "action_start_focus"

    def run(self, dispatcher: CollectingDispatcher,
            tracker: Tracker,
            domain: Dict[Text, Any]) -> List[Dict[Text, Any]]:
        duration_entity = next(tracker.get_latest_entity_values("duration"), None)
        if duration_entity:
            try:
                minutes = int(duration_entity)
                r = requests.post(f"{RUST_API}/start", json={"minutes":minutes}, timeout=3)
                if r.ok:
                    dispatcher.utter_message(f"Starting a {minutes}-minute focus session. Good luck!")
                else:
                    dispatcher.utter_message("A session is already running!")
            except Exception as e:
                dispatcher.utter_message(text=f"Error starting session: {str(e)}")
        else:
            dispatcher.utter_message(text="I coudln't figure out the focus duration. Try again?")    

        return []

class ActionCheckStatus(Action):
    def name(self) -> Text:
        return "action_check_status"
    
    def run(self, dispatcher: CollectingDispatcher,
            tracker: Tracker,
            domain: Dict[Text, Any]) -> List[Dict[Text, Any]]:
        try:
            r = requests.get(f"{RUST_API}/status", timeout=3)
            if r.ok:
                data = r.json()
                remaining = data.get("minutes_remaining", "unknown")
                state = data.get("state", "unknown")
                
                msg = f"🍵 Focus session is currently *{state}* with *{remaining}* minutes remaining."
                dispatcher.utter_message(text=msg)
            else:
                dispatcher.utter_message(text="I coudln't get that session information. Is CharGuard running?")
        except Exception as e:
            dispatcher.utter_message(text=f"Error checking session: {str(e)}")
        return []
                