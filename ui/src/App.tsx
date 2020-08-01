import React from "react";
import Video from "./components/video";
import Settings from "./components/settings";
import "./style.css";

const App = () => (
  <div className="App">
    <div>
      <Settings />
    </div>
    <div>
      <Video />
    </div>
  </div>
);

export default App;
