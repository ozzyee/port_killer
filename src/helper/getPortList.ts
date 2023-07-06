import {invoke} from "@tauri-apps/api/tauri";

type PortList = {
  name: string
  pid: string
  CP: string
  status: string
  port: string
}

export const getPortList = async (): Promise<PortList[]> => {
  const res = await invoke("get_current_running_ports") as string
  const data = JSON.parse(res)
  return data.map((_item: string) => {
    const item = _item.split(" ").filter((element) => element !== "")

    return {
      name: item[0],
      pid: item[1],
      CP: item[7],
      status: item[9],
      port: item[8]
    }
  })
}