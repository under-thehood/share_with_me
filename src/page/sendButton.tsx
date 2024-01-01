import "./send.css";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

export default function SendButton({ setRoute }: any) {
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
  return (
    <div>
      <button onClick={setUpSendingProcess}>
        <div className="svg-wrapper-1">
          <div className="svg-wrapper">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              width="24"
              height="24"
            >
              <path fill="none" d="M0 0h24v24H0z"></path>
              <path
                fill="currentColor"
                d="M1.946 9.315c-.522-.174-.527-.455.01-.634l19.087-6.362c.529-.176.832.12.684.638l-5.454 19.086c-.15.529-.455.547-.679.045L12 14l6-8-8 6-8.054-2.685z"
              ></path>
            </svg>
          </div>
        </div>
        <span>Send</span>
      </button>
    </div>
  );
}
