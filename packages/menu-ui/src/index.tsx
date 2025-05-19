/* @refresh reload */
import './index.css';
import { render } from 'solid-js/web';
import { createSignal, createEffect, onCleanup } from 'solid-js';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { setForegroundWindow, shellExec, resizeMenu, hideMenu } from 'edgebar';

type DropDownItem = {
  name: string;
  action: string;
  hwnd?: number;
  icon?: string;
  key?: string;
  disabled?: boolean;
};

const FONT_SIZE = 14;
const MENU_PADDING = 10;

function WidgetDropDown() {
  const [items, setItems] = createSignal<DropDownItem[]>([]);

  // Listen for updates to the menu items
  const unlisten: Promise<UnlistenFn> = listen<DropDownItem[]>('updateMenuItems', (event) => {
    setItems(event.payload);
  });

  // Cleanup listener on component unmount
  onCleanup(async () => {
    (await unlisten)();
  });

  // Once items update, recalc the width based on the longest text and then call resizeMenu.
  createEffect(() => {
    const currentItems = items();
    requestAnimationFrame(() => {
      const body = document.body;
      if (body) {
        const rect = body.getBoundingClientRect();
        // Calculate the width based on the longest text (button name and key) plus icon width.
        let maxTextWidth = currentItems.reduce((maxWidth, item) => {
          const nameWidth = item.name ? item.name.length * FONT_SIZE * 0.7 : 0;
          const keyWidth = item.key ? item.key.length * FONT_SIZE * 0.5 : 0;
          // If an icon exists, assume a fixed width of 20.
          const iconWidth = item.icon ? 10 : 0;
          const total = nameWidth + keyWidth + iconWidth;
          return Math.max(maxWidth, total);
        }, 0);

        const computedWidth = maxTextWidth + MENU_PADDING;
        // Call resizeMenu from your backend (which will convert logical to physical pixels).
        resizeMenu(computedWidth, rect.height);
      }
    });
  });

  const handleAction = async (action: string, hwnd?: number) => {
    if (hwnd) {
      hideMenu();
      setForegroundWindow(hwnd);
    }
    await shellExec('powershell', ['-Command', action]);
  };

  return (
    <ul class="dropdown">
      {items().map(item =>
        item.name === 'spacer' ? (
          <li class="spacer"></li>
        ) : (
          <li class={`${item.disabled ? 'disabled' : ''}`}>
            <button
              onClick={() => handleAction(item.action, item.hwnd)}
            >
              {item.name}
            </button>
            {item.icon && (
              <>
                {item.icon.split(',').map((ic, index) => {
                  const trimmed = ic.trim();
                  const iconClass = `nf ${trimmed}`;
                  return (
                    <i
                      key={index}
                      class={iconClass}
                      onClick={() => handleAction(item.action, item.hwnd)}
                    />
                  );
                })}
                <span>{item.key}</span>
              </>
            )}
          </li>
        ),
      )}
    </ul>
  );
}

render(() => <WidgetDropDown />, document.getElementById('root')!);