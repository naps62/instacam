import React, { useState, useEffect } from "react";

import Button from "@material-ui/core/Button";

import axios from "axios";
import "./style.css";

const fetchSettings = async (callback: Function) => {
  try {
    const { data } = await axios("http:///localhost:8000/api/settings");

    callback(data);
  } catch (err) {
    console.error(err);
  }
};

const updateSettings = async (settings: String) => {
  try {
    await axios({
      method: "post",
      url: "http://localhost:8000/api/settings",
      data: settings,
    });
  } catch (err) {
    console.error(err);
  }
};

const Settings: React.FC = () => {
  const [settings, setSettings] = useState("");

  useEffect(() => {
    fetchSettings((settings: JSON) =>
      setSettings(JSON.stringify(settings, undefined, 2))
    );
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
        rows={30}
        className="Settings"
        defaultValue={settings}
        onChange={(e) => setSettings(e.target.value)}
      />
      <Button variant="contained" color="primary" type="submit">
        Save
      </Button>
    </form>
  );
};

export default Settings;
