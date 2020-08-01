import React, { useState, useEffect } from "react";
import { useWasm, useTakeEffect } from "../../utils/wasm";
import "./style.css";

const Settings: React.FC = () => {
  const wasm = useWasm();
  const [settings, setSettings] = useState("");

  // useTakeEffect(() => {
  //   wasm.set_settings("asd");
  // }, [wasm]);

  useTakeEffect(() => {
    setSettings("{}");
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
