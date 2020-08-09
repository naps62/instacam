import React from "react";
// import axios from "axios";
// import "./style.css";

// const setFilters = async (callback: Function) => {
//   try {
//     const { data } = await axios("http:///localhost:8000/api/settings");

//     callback(data);
//   } catch (err) {
//     console.error(err);
//   }
// };

// const updateSettings = async (settings: String) => {
//   try {
//     await axios({
//       method: "post",
//       url: "http://localhost:8000/api/settings",
//       data: settings,
//     });
//   } catch (err) {
//     console.error(err);
//   }
// };

const Settings: React.FC = () => {
  // const [settings, setSettings] = useState("");

  // useEffect(() => {
  //   fetchSettings((settings: JSON) =>
  //     setSettings(JSON.stringify(settings, undefined, 2))
  //   );
  // }, []);

  // const onSubmit = (e: any) => {
  //   e.preventDefault();

  //   try {
  //     const json = JSON.parse(settings);
  //     updateSettings(json);
  //   } catch (err) {
  //     console.error(err);
  //   }
  // };

  return (
    <form>
      <div>
        <input type="radio" id="pixelate" />
        <label htmlFor="pixelate">Pixel</label>
      </div>
      <input type="submit" value="Save" />
    </form>
  );
};

export default Settings;
