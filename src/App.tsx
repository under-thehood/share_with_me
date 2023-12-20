// import { useState } from "react";
// import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import "./App.css";
import DashBoard from "./page/DashBoard";
import SendPage from "./page/Send";
import RecievePage from "./page/Recieve";

function App() {
  const [currentPage, setCurrentPage] = useState("/");

  switch (currentPage) {
    case "/":
      return <DashBoard setRoute={setCurrentPage} />;
    case "/send":
      return <SendPage setRoute={setCurrentPage} />;
    case "/recieve":
      return <RecievePage setRoute={setCurrentPage} />
    default:
      return <DashBoard setRoute={setCurrentPage} />;
  }
}

export default App;
