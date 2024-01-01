import React, { useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

interface SendPageProps {
  setRoute: React.Dispatch<React.SetStateAction<string>>;
}

interface RecieverInfo {
  message: string;
}

function getRandomIntString(min: number, max: number) {
  // The maximum is exclusive and the minimum is inclusive
  return Math.floor(Math.random() * (max - min)) + min.toString();
}

// Functional Component
const SendPage: React.FC<SendPageProps> = () => {
  const [popUpArray, setPopUpArray] = useState([""]);

  listen<RecieverInfo>("new_reciever", (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object

    let data = event.payload.message;

    setPopUpArray([...popUpArray, data]);
  }).then(() => {
    console.log("New reciever is found!!");
    console.log(popUpArray);
  });

  return (
    <div className="container">
      <div className="sender-avatar ripple-blue">Sender</div>
      {popUpArray.length > 1 ? (
        <div
          className="reciever-popup"
          style={{
            visibility: "visible",
            top: getRandomIntString(100, 200),
            right: getRandomIntString(100, 200),
          }}
          onClick={() => {
            invoke("ask_reciever_permission", {
              receiverInfo: { message: popUpArray[1] },
            });
          }}
        >
          {popUpArray[1]}
        </div>
      ) : (
        <div className="reciever-avatar" style={{ visibility: "hidden" }}></div>
      )}
    </div>
  );
};

export default SendPage;
