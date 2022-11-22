import { appWindow } from "@tauri-apps/api/window";
import { TbLayersIntersect } from "solid-icons/tb";
import { BsDashLg } from "solid-icons/bs";
import { IoClose } from "solid-icons/io";

const COLOR = "#f1f5f9";

function WindowTitle() {
  return (
    <>
      <div
        onMouseDown={(e) => {
          if (e.target.tagName === "DIV") {
            appWindow.startDragging();
          } else {
            return;
          }
        }}
        class="bg-slate-700 flex w-full h-9 z-0 place-content-end"
      >
        <button
          class="w-8 hover:bg-neutral-50 hover:bg-opacity-25 flex flex-row justify-center place-items-center hover:cursor-default"
          onClick={() => appWindow.minimize()}
        >
          <BsDashLg size={20} color={COLOR} />
        </button>
        <button
          class="w-8 hover:bg-neutral-50 hover:bg-opacity-25 flex flex-row justify-center place-items-center hover:cursor-default"
          onClick={() => appWindow.toggleMaximize()}
        >
          <TbLayersIntersect size={20} color={COLOR} />
        </button>
        <button
          onClick={() => appWindow.close()}
          class=" w-8 hover:bg-red-500 flex flex-row justify-center place-items-center hover:cursor-default"
        >
          <IoClose size={20} color={COLOR} />
        </button>
      </div>
    </>
  );
}

export default WindowTitle;
