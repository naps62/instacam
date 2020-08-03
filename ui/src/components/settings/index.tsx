import React, { useState, useEffect } from "react";
import axios from "axios";
import "./style.css";

const fetchSettings = async (callback: Function) => {
  try {
    const { data } = await axios("http:///localhost:8000/settings");

    callback(data);
  } catch (err) {
    console.error(err);
  }
};

const updateSettings = async (settings: String) => {
  try {
    await axios({
      method: "post",
      url: "http://localhost:8000/settings",
      data: settings,
    });
  } catch (err) {
    console.error(err);
  }
};

const Settings: React.FC = () => {
  const [settings, setSettings] = useState("");

  useEffect(() => {
    fetchSettings((settings: JSON) => setSettings(JSON.stringify(settings)));
  }, []);

  const onSubmit = (e: any) => {
    e.preventDefault();

    try {
      const json = JSON.parse(settings);
      updateSettings(json);
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <form onSubmit={onSubmit}>
      <textarea
        rows={20}
        className="Settings"
        defaultValue={settings}
        onChange={(e) => setSettings(e.target.value)}
      />
      <input type="submit" value="Save" />
    </form>
  );
};

export default Settings;
