import React, { useState } from 'react';
import { emit, listen } from "@tauri-apps/api/event";


interface RecievePageProps {
    setRoute: React.Dispatch<React.SetStateAction<string>>;
};


interface RecieverInfo {
    message: string;
}

// Functional Component
const RecievePage: React.FC<RecievePageProps> = () => {




    let [senderName, setSenderName] = useState("");

    // let [connectResponse, setConnectResponse] = useState("reject");



    listen<RecieverInfo>("connect_attempt", (event) => {
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object


        let data = event.payload.message;

        console.log("connect-attempt event happened");


        setSenderName(data);

        const dialog = document.querySelector("dialog");
        if (!dialog) {
            console.log("dialor is not selected");
        }

        dialog?.showModal();
    })

    return (
        <div>
            <div className='ripple-green reciever-avatar'>Reciever</div>
            <dialog>
                <span> {senderName} is trying to connect . Do you want to accept or reject?</span>
                <button onClick={() => {
                    emit("connect_status_response", { messgae: 'connect accepted' });
                    const dialog = document.querySelector("dialog");
                    dialog?.close();
                }}>Accept</button>
                <button onClick={() => emit("connect_status_response", { messgae: 'connect rejected' })} >Reject</button>
            </dialog>
            Reciever is broadcasting!!
        </div >


    );
};

export default RecievePage;