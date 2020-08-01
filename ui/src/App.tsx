import React from "react";
import logo from "./logo.svg";
import { useWasm, useTakeEffect } from "./utils/hooks";
import "./App.css";

const App: React.FC = () => {
  const mod = useWasm();
  const [response, setResponse] = React.useState();

  useTakeEffect(() => {
    const resp = mod.greet("Hello", ["from", "TypeScript"]);
    setResponse(resp);
  }, [mod]);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <p>Response: {response}</p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
};

export default App;
