import React, { useState, useCallback, useEffect } from "react";

import Switch from "@material-ui/core/Switch";

import Webcam from "react-webcam";
import "./style.css";

const Video: React.FC = () => {
  const [device, setDevice] = useState<any>(null);
  const [enabled, setEnabled] = useState(false);

  const handleDevices = useCallback(
    (devices) =>
      setDevice(
        devices.find(
          ({ kind, label }: any) =>
            kind === "videoinput" && label === "Instacam"
        )
      ),
    [setDevice]
  );

  useEffect(() => {
    navigator.mediaDevices.enumerateDevices().then(handleDevices);
  }, [handleDevices]);

  let webcam = null;
  if (enabled && device) {
    webcam = (
      <Webcam
        videoConstraints={{
          deviceId: device.deviceId,
          width: 800,
          height: 600,
        }}
        mirrored={true}
        audio={false}
      />
    );
  }

  return (
    <div className="Video">
      <Switch
        checked={enabled}
        onChange={(e: any) => setEnabled(e.target.checked)}
        color="primary"
        inputProps={{ "aria-label": "primary checkbox" }}
      />

      {webcam}
    </div>
  );
};

export default Video;
