import { appWindow } from "@tauri-apps/api/window";
import { TbLayersIntersect } from "solid-icons/tb";
import { BsDashLg } from "solid-icons/bs";
import { IoClose } from "solid-icons/io";

const COLOR = "#1C1C28";

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
        class="fixed top-0 z-0 flex h-7 w-full place-content-end bg-white"
      >
        <button
          class="flex w-8 flex-row place-items-center justify-center hover:cursor-default hover:bg-[#1C1C28] hover:bg-opacity-25"
          onClick={() => appWindow.minimize()}
        >
          <BsDashLg size={20} color={COLOR} />
        </button>
        <button
          class="flex w-8 flex-row place-items-center justify-center hover:cursor-default hover:bg-[#1C1C28] hover:bg-opacity-25"
          onClick={() => appWindow.toggleMaximize()}
        >
          <TbLayersIntersect size={20} color={COLOR} />
        </button>
        <button
          onClick={() => appWindow.close()}
          class=" flex w-8 flex-row place-items-center justify-center hover:cursor-default hover:bg-red-500"
        >
          <IoClose size={20} color={COLOR} />
        </button>
      </div>
    </>
  );
}

export default WindowTitle;
