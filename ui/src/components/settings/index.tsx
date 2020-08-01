import React, { useState, useEffect } from "react";
import * as Wasm from "../../wasm";
import "./style.css";

const Settings: React.FC = () => {
  const wasm = Wasm.use();
  const [settings, setSettings] = useState("");

  Wasm.useEffect(() => {
    wasm.set_settings("asd");
  }, [wasm]);

  const onSubmit = (e: any) => {
    console.log(e);
    wasm.setSettings(settings);
  };

  return (
    <form onSubmit={onSubmit}>
      <textarea
        rows={20}
        className="Settings"
        value={JSON.stringify(settings, null, 2)}
      />
      <input type="submit" value="Save" />
    </form>
  );
};

export default Settings;
