import React, { useState, useCallback, useEffect } from "react";
import Webcam from "react-webcam";
import "./style.css";

const Video: React.FC = () => {
  const [deviceId, setDeviceId] = useState("");
  const [devices, setDevices] = useState([]);

  const handleDevices = useCallback(
    (devices) =>
      setDevices(
        devices.filter(
          ({ kind, label }: any) =>
            kind === "videoinput" && label === "Instacam"
        )
      ),
    [setDevices]
  );

  useEffect(() => {
    navigator.mediaDevices.enumerateDevices().then(handleDevices);
  }, [handleDevices]);

  let webcam = null;
  if (deviceId !== null && deviceId !== "") {
    webcam = (
      <Webcam
        videoConstraints={{ deviceId: deviceId, width: 800, height: 600 }}
        mirrored={true}
        audio={false}
      />
    );
  }

  return (
    <div className="Video">
      <select onChange={(e: any) => setDeviceId(e.target.value)}>
        <option value="">None</option>
        {devices.map((device: any, key: any) => (
          <option key={key} value={device.deviceId}>
            {device.label}
          </option>
        ))}
      </select>
      {webcam}
    </div>
  );
};

export default Video;
