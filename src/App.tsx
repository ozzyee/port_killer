import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.scss";
import {getPortList} from "./helper/getPortList";

function App() {
  const [port, setPorts] = useState([]);
  const [button, setButton] = useState<any>({
    buttonId: "",
    clicked: false,
    color: ""
  });

  useEffect(() => {
    const timer = setInterval(()=>{
      getPortList().then((ports) => {
        setPorts(ports as any);
      });
    }, 1000);

    return () => {
      clearInterval(timer);
    };
  }, []);

  const killPort = (buttonId: number, pid: string) => {
    if (buttonId === button.buttonId) {
      setButton((prevButton: { clicked: any; }) => ({
        ...prevButton,
        clicked: !prevButton.clicked,
        color: prevButton.clicked ? "" : "red"
      }));
      invoke("kill_port", {pid: Number(pid)}).then((res) => {
        if (res) {
          setButton({
            buttonId: "",
            clicked: false,
            color: ""
          });
          getPortList().then((ports) => {
            setPorts(ports as any);
          });
        }
      });
      return;
    }

    setButton({
      buttonId: buttonId,
      clicked: true,
      color: "red"
    });
  };

  return (
    <div className="main-container">
      <div className="container"/>
      <div className="content">
        <h1>Port killer</h1>
        <div>
          <table>
            <thead>
            <tr>
              <th>Name</th>
              <th>PID</th>
              <th>Port</th>
              <th></th>
            </tr>
            </thead>
            <tbody>
            {port.map((item: any, index: number) => (
              <tr key={index}>
                <td>{item.name}</td>
                <td>{item.pid}</td>
                <td>{item.port}</td>
                <td>
                  <button
                    style={{
                      backgroundColor:
                        button.clicked && button.buttonId === index
                          ? button.color
                          : ""
                    }}
                    onClick={() => {
                      killPort(index, item.pid);
                    }}
                  >
                    Cancel Process
                  </button>
                </td>
              </tr>
            ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

export default App;
