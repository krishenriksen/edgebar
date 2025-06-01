import "./index.css";
import { render } from "solid-js/web";
import { createStore } from "solid-js/store";
import { createSignal, createEffect, createMemo } from "solid-js";
import { currentMonitor } from "@tauri-apps/api/window";
import { createProviderGroup, shellExec, showMenu, hideMenu } from "edgebar";

// Use Vite's import.meta.glob to import all files dynamically
const modules = import.meta.glob("./applications/*.ts", { eager: true });
const Applications: Record<string, any> = {};
Object.keys(modules).forEach((filePath) => {
  const moduleName = filePath.replace(/.*\/(.*)\.ts$/, "$1");
  Applications[moduleName] = modules[filePath];
});

function App() {
  const providers = createProviderGroup({
    audio: { type: "audio" },
    systray: { type: "systray" },
    window: { type: "window" },
  });

  const [output, setOutput] = createStore(providers.outputMap);
  createEffect(() => providers.onOutput(setOutput));

  /**
   * signal to track if the monitor is wide enough
   * to show the full taskbar
   */
  const [isWideEnough, setIsWideEnough] = createSignal(true);

  createEffect(async () => {
    const monitor = await currentMonitor();
    setIsWideEnough((monitor?.size?.width || window.innerWidth) >= 1280);
  });

  /**
   * Show taskbar context menu on right click
   */
  const taskbarMenuItems = [
    {
      name: "Task Manager",
      action: 'start taskmgr',
      icon: 'nf-md-chart_bar',
      hwnd: 0,
    },
    {
      name: "Taskbar settings",
      action: 'start ms-settings:taskbar',
      icon: 'nf-md-cog',
      hwnd: 0,
    },
  ];

  createEffect(() => {
    const handler = (e: MouseEvent) => {
      e.preventDefault();

      // Prevent menu if a systray icon was clicked
      let el = e.target as HTMLElement | null;
      while (el) {
        if (el.classList && el.classList.contains("systray-icon")) {
          return; // Don't show the menu
        }
        el = el.parentElement;
      }

      // Use document.body as the anchor for the menu
      const target = document.body as HTMLButtonElement;
      const rect = target.getBoundingClientRect();

      // Create a proxy event that mimics MouseEvent and adds getBoundingClientRect, x, and y
      const customEvent = new Proxy(e, {
        get(obj, prop) {
          if (prop === "currentTarget") {
            // Return a mock element with getBoundingClientRect
            return {
              getBoundingClientRect: () => ({
                x: e.x,
                y: e.y,
                width: rect.width,
                height: rect.height,
                left: e.x,
                top: 0,
                right: e.x + rect.width,
                bottom: e.y + rect.height
              }),
            };
          }

          // Fallback to original event properties
          return (obj as any)[prop];
        },
      }) as MouseEvent;

      handleMenuInteraction(customEvent, taskbarMenuItems);
    };

    document.addEventListener("contextmenu", handler);
    return () => document.removeEventListener("contextmenu", handler);
  });

  /**
   * Get menu entries for specific applications
   */
  type MenuItem = {
    name: string;
    action: string;
    hwnd: number;
    icon?: string | null;
    key?: string | null;
    disabled?: boolean;
  };

  type MenuGroup = {
    name: string;
    items: MenuItem[];
  };

  type DropdownOption = {
    name: string;
    items?: MenuItem[];
  };

  type ModuleType = {
    applicationTitles: string[];
    menuItems: {
      name: string;
      items: {
        name: string;
        action: string;
        hwnd: number;
        icon?: string | null;
        key?: string | null;
        disabled?: boolean;
      }[];
    }[];
  };

  const [isMenuVisible, setMenuVisible] = createSignal(false);
  const [activeMenuName, setActiveMenuName] = createSignal<string | null>(null);
  const windowsModule = Applications["Windows"];
  const fileExplorerModule = Applications["FileExplorer"];
  const [appSpecificOptions, setAppSpecificOptions] = createSignal<
    DropdownOption[]
  >([
    ...(windowsModule
      ? windowsModule.menuItems.map((menuGroup: MenuGroup) => ({
          name: menuGroup.name,
          items: menuGroup.items.map((item: MenuItem) => ({
            name: item.name,
            action: item.action,
            hwnd: 0,
            icon: item.icon || null,
            key: item.key || null,
            disabled: item.disabled || false,
          })),
        }))
      : []),
    ...(fileExplorerModule
      ? fileExplorerModule.menuItems.map((menuGroup: MenuGroup) => ({
          name: menuGroup.name,
          items: menuGroup.items.map((item: MenuItem) => ({
            name: item.name,
            action: item.action,
            hwnd: 0,
            icon: item.icon || null,
            key: item.key || null,
            disabled: item.disabled || false,
          })),
        }))
      : []),
  ]);
  const defaultTitle = "File Explorer";
  const replaceTitle = ["Program Manager"];

  createEffect(async () => {
    let title = replaceTitle.includes(output.window?.title)
      ? defaultTitle
      : output.window?.title;

    if (title && output.window?.hwnd) {
      // Extract the last part of the title or fallback to the full title
      title = title
        .replace(" – ", " - ") // En-dash
        .replace(" — ", " - ") // Em-dash
        .split(" - ")
        .filter((s: string) => s.trim() !== "")
        .pop()
        ?.trim();

      const hwnd = parseInt(output.window.hwnd);

      // close the current menu
      hideMenu();
      setMenuVisible(false);
      setActiveMenuName(null);

      let options: DropdownOption[] = [];

      // Check if the module exists in Applications
      const module: ModuleType | undefined = Object.values(Applications).find(
        (mod) => mod.applicationTitles && mod.applicationTitles.includes(title),
      );

      if (module) {
        module.menuItems.forEach((section: MenuGroup) => {
          options.push({
            name: section.name,
            items: section.items.map((item: MenuItem) => ({
              name: item.name,
              action: item.action,
              hwnd: hwnd,
              icon: item.icon || null,
              key: item.key || null,
              disabled: item.disabled || false,
            })),
          });
        });
      } else {
        // If the module is not found, add only the title
        options.push({
          name: title,
        });
      }

      // Only keep the first option if monitor is narrow
      if (!isWideEnough()) {
        options = options.slice(0, 1);
      }

      // Update appSpecificOptions while preserving the Windows module at index 0
      setAppSpecificOptions((prevOptions) => {
        const firstOption = prevOptions[0];
        return [firstOption, ...options];
      });
    }
  });

  const handleMenuInteraction = async (e: MouseEvent, items: MenuItem[]) => {
    const target = e.currentTarget as HTMLButtonElement;
    const rect = target.getBoundingClientRect();

    const monitorInfo = await currentMonitor();

    showMenu(
      items,
      { x: rect.left, y: rect.top, width: rect.width, height: rect.height },
      monitorInfo?.position || { x: 0, y: 0 },
    );
  };

  /**
   * set volume
   */
  const [isVolumeVisible, setVolumeVisible] = createSignal(false);
  const [isVolumeHovered, setVolumeHovered] = createSignal(false);
  let volumeInteval: number | undefined;

  const handleVolumeEnter = () => {
    setVolumeVisible(true);
    setVolumeHovered(true);
    if (volumeInteval) {
      clearTimeout(volumeInteval);
      volumeInteval = undefined;
    }
  };

  const handleVolumeMove = (e: any) => {
    // Called on mouse move, ensuring the slider stays visible while moving.
    setVolumeVisible(true);
    if (volumeInteval) {
      clearTimeout(volumeInteval);
      volumeInteval = undefined;
    }
    if (!isVolumeHovered()) {
      volumeInteval = setTimeout(() => {
        if (!isVolumeHovered()) setVolumeVisible(false);
      }, 1000);
    }
  };

  const handleVolumeLeave = () => {
    setVolumeHovered(false);
    volumeInteval = setTimeout(() => {
      if (!isVolumeHovered()) {
        setVolumeVisible(false);
      }
    }, 1000);
  };

  /**
   * Renders the icons in the system tray.
   */
  type SystrayIcon = {
    id: string;
    iconUrl: string;
    tooltip: string;
  };

  interface IconCacheValue {
    element: HTMLLIElement;
  }

  const iconCache: Map<string, IconCacheValue> = new Map();

  const SystrayIcons = createMemo(() => {
    if (output.systray) {
      // remove icons that are not in the current output
      const currentIds = new Set(
        output.systray.icons.map((icon: { id: any }) => icon.id),
      );
      iconCache.forEach((_, id) => {
        if (!currentIds.has(id)) {
          iconCache.delete(id);
        }
      });

      return output.systray.icons
        .filter(
          (icon: SystrayIcon) =>
            !icon.tooltip?.toLowerCase().includes("speakers") &&
            !icon.tooltip?.toLowerCase().includes("edgebar"),
        )
        .sort((a: SystrayIcon, b: SystrayIcon) => {
          const getPriority = (icon: SystrayIcon) => {
            const tooltip = icon.tooltip?.toLowerCase() || "";
            if (tooltip.includes("cpu core")) return 1;
            if (tooltip.includes("gpu")) return 2;
            return 99; // everything else gets a lower priority
          };

          return getPriority(a) - getPriority(b);
        })
        .map((icon: SystrayIcon) => {
          if (iconCache.has(icon.id)) {
            const cachedIcon = iconCache.get(icon.id);
            if (cachedIcon) {
              const img = cachedIcon.element.querySelector("img");
              if (img) {
                img.src = icon.iconUrl;
                img.title = icon.tooltip;
              }
            }
          } else {
            const li = (
              <li
                id={icon.id}
                class="systray-icon"
                onClick={(e: MouseEvent) => {
                  e.preventDefault();
                  output.systray?.onLeftClick(icon.id);
                }}
                onContextMenu={(e: MouseEvent) => {
                  e.preventDefault();
                  output.systray?.onRightClick(icon.id);
                }}
              >
                <img src={icon.iconUrl} title={icon.tooltip} />
              </li>
            );

            iconCache.set(icon.id, { element: li as HTMLLIElement });
          }

          return iconCache.get(icon.id)?.element;
        });
    }

    return null;
  });

  /**
   * Add local date state updated using JavaScript
   */
  const [currentDate, setCurrentDate] = createSignal(new Date());
  createEffect(() => {
    const interval = setInterval(() => setCurrentDate(new Date()), 60000);
    return () => clearInterval(interval);
  });

  const formatShortDate = (date: Date): string => {
    // Build the date part manually to enforce the order "weekday day month"
    const weekday = new Intl.DateTimeFormat(navigator.language, {
      weekday: "short",
    }).format(date);
    const day = date.getDate();
    const month = new Intl.DateTimeFormat(navigator.language, {
      month: "short",
    }).format(date);
    const datePart = `${weekday} ${day} ${month}`;

    // Format time in 24-hour format.
    const hours = date.getHours();
    const minutes = date.getMinutes();
    const hourStr = hours < 10 ? `0${hours}` : hours.toString();
    const minuteStr = minutes < 10 ? `0${minutes}` : minutes.toString();
    const timePart = `${hourStr}:${minuteStr}`;

    // Use a better padding character (e.g., two em-spaces)
    const padding = "\u2002";
    return `${datePart}${padding}${timePart}`;
  };

  const formatLongDate = (date: Date): string =>
    new Intl.DateTimeFormat(navigator.language, {
      weekday: "long",
      day: "numeric",
      month: "long",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    })
      .format(date)
      .replace(/,/g, "");

  function LeftPanel() {
    return (
      <div class="left">
        <ul>
          {appSpecificOptions().map(({ name, items }: any, index: number) => (
            <li key={index}>
              <button
                class={`${
                  index === 0 ? "nf nf-fa-windows" : ""
                } ${isMenuVisible() && activeMenuName() === name ? "active" : ""}`}
                onClick={(e: MouseEvent) => {
                  if (isMenuVisible()) {
                    hideMenu();
                    setActiveMenuName(null);
                  } else {
                    setActiveMenuName(name);
                    handleMenuInteraction(e, items || []);
                  }
                  setMenuVisible(!isMenuVisible());
                }}
                onMouseEnter={async (e: MouseEvent) => {
                  if (isMenuVisible() && activeMenuName() !== name) {
                    setActiveMenuName(name);
                    handleMenuInteraction(e, items || []);
                  }
                }}
              >
                {name}
              </button>
            </li>
          ))}
        </ul>
      </div>
    );
  }

  function RightPanel() {
    return (
      <div class="right">
        <ul>
          {output.audio?.defaultPlaybackDevice && (
            <li>
              <button
                title={`Volume: ${output.audio.defaultPlaybackDevice.volume}`}
                class={`volume nf ${
                  output.audio.defaultPlaybackDevice.volume === 0
                    ? "nf-fa-volume_xmark"
                    : output.audio.defaultPlaybackDevice.volume < 20
                      ? "nf-fa-volume_low"
                      : output.audio.defaultPlaybackDevice.volume < 40
                        ? "nf-fa-volume_low"
                        : "nf-fa-volume_high"
                }`}
                onMouseEnter={handleVolumeEnter}
                onMouseMove={handleVolumeMove}
                onMouseLeave={handleVolumeLeave}
              ></button>
              <input
                title={`Volume: ${output.audio.defaultPlaybackDevice.volume}`}
                class={`${isVolumeVisible() ? "active" : ""}`}
                type="range"
                min="0"
                max="100"
                step="2"
                value={output.audio.defaultPlaybackDevice.volume}
                onInput={(e) => output.audio?.setVolume(e.target.valueAsNumber)}
                onMouseEnter={handleVolumeEnter}
                onMouseMove={handleVolumeMove}
                onMouseLeave={handleVolumeLeave}
              />
            </li>
          )}

          {SystrayIcons()}

          {isWideEnough() && (
            <li title={formatLongDate(currentDate())}>
              <button
                class="date"
                onClick={() => {
                  shellExec("powershell", ["-Command", "start ms-actioncenter:"]);
                }}
              >
                {formatShortDate(currentDate())}
              </button>
            </li>
          )}
        </ul>
      </div>
    );
  }

  return (
    <div class="app">
      <LeftPanel />
      <RightPanel />
    </div>
  );
}

render(() => <App />, document.getElementById("root")!);
