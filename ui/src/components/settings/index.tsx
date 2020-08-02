import React, { useState, useEffect } from "react";
import "./style.css";

const Settings: React.FC = () => {
  const [wasm, setWasm] = useState();
  const [settings, setSettings] = useState("");

  useEffect(() => {
    (async () => {
      const wasm = await import("instacam-wasm");
      setWasm(wasm);
    })();
  });

  useEffect(() => {
    if (!wasm) return;

    console.log(wasm.get_settings());
  }, [wasm]);

  const onSubmit = (e: any) => {
    e.preventDefault();
    wasm.set_settings(settings);
  };

  if (!wasm) {
    return <div>Loading</div>;
  } else {
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
  }
};

export default Settings;
