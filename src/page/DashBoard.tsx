import React from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";



interface DashBoardProps {
    setRoute: React.Dispatch<React.SetStateAction<string>>;
};


// Functional Component
const DashBoard: React.FC<DashBoardProps> = ({ setRoute }) => {


    async function setUpSendingProcess() {
        // let fileSelector = event.target as HTMLInputElement;

        // let files = fileSelector.files;
        let file = await open({ multiple: false, title: "Select file" });



        if (file) {

            setRoute("/send");
            // const fileNames = Array.from(files).map(file => file.name);

            try {
                await invoke("setup_sender", { files: [file] });
            } catch (error) {
                console.error("Error invoking setup_sender:", error);
            }


        } else {
            console.log("cannot read the file");
        }
    }



    function setUpRecievingProcess() {

        invoke("setup_reciever");

        setRoute("/recieve");
    }


    return (<div className="container">
        <div className="row">
            <div className="send-button"><button className='color-blue' onClick={setUpSendingProcess} /><span>Send</span></div>
            <span className="color-green" onClick={setUpRecievingProcess}>Recieve</span>
        </div>
    </div >);
};


// onClick={() => setRoute("/send")}
export default DashBoard;