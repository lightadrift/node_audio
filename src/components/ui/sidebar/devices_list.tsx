import { invoke } from "@tauri-apps/api/tauri";
import { createResource, createSignal, For, Match, Switch } from "solid-js";
import { VsRefresh } from "solid-icons/vs";


interface Devices {
  name: string;
  id: string;
}

async function get_data() {
  const data: string = await invoke("get_data_devices");
  const res = await JSON.parse(data);
  return res;
}

const Devices = () => {
  // const [refetch_devices, setRefetch] = createSignal();
  const [data, { refetch }] = createResource<Devices[]>(get_data, {
    initialValue: [],
  });

  return (
    <>
      <div class=" h-full  w-1/6 bg-[#28293D] text-[#EBEBF0]">
        <div>{data.loading && "loading"}</div>
        <div>
          <For each={data()}>
            {(device) => (
              <div class="p-2 text-xs font-normal">{device.name}</div>
            )}
          </For>
        </div>
      </div>
    </>
  );
};

export default Devices;
