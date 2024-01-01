import React from "react";
import SendButton from "./sendButton";
import RecieveButton from "./recieveButton";
import "./dashBoard.css";

interface DashBoardProps {
  setRoute: React.Dispatch<React.SetStateAction<string>>;
}

// Functional Component
const DashBoard: React.FC<DashBoardProps> = ({ setRoute }) => {
  return (
    <div className="dashboardContainer">
      <div className="dashboardWrapper">
        <div className="sendButton">
          <SendButton setRoute={setRoute} />
        </div>
        <div className="receiveButton">
          <RecieveButton setRoute={setRoute} />
        </div>
      </div>
    </div>
  );
};

// onClick={() => setRoute("/send")}
export default DashBoard;
