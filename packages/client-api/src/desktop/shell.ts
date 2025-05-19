import { listen, type Event } from '@tauri-apps/api/event';
import {
  desktopCommands,
  type ShellCommandOptions,
  type ShellExecOutput,
} from './desktop-commands';

interface ShellEmission {
  pid: number;
  event: ShellEvent;
}

type ShellEvent<T extends string | Uint8Array = string> =
  | {
      type: 'stdout';
      data: T;
    }
  | {
      type: 'stderr';
      data: T;
    }
  | {
      type: 'error';
      data: string;
    }
  | {
      type: 'terminated';
      data: {
        exitCode: number | null;
        signal: number | null;
      };
    };

/**
 * Sets the specified window as the foreground window.
 *
 * @param {number} hwnd - The handle of the window to bring to the foreground.
 * @throws - If the operation fails.
 */
export async function setForegroundWindow(hwnd: number): Promise<void> {
  return await desktopCommands.setForegroundWindow(hwnd);
}

/**
 *  Show a menu at the specified position.
 */
export async function showMenu(
  subItems: {
    name: string;
    action: string;
    hwnd: number;
    icon?: string;
    key?: string;
    disabled?: boolean;
  }[],
  button: { x: number; y: number, width: number, height: number },
  monitor: { x: number; y: number }
): Promise<void> {
  return await desktopCommands.showMenu(subItems, button, monitor);
}

/**
 * Resize the menu height.
 *
 * @param {number} height - The new height of the menu.
 * @throws - If the operation fails.
 */
export async function resizeMenu(width: number, height: number): Promise<void> {
  return await desktopCommands.resizeMenu(width, height);
}

/**
 * Close menu based on application.
 *
 * @throws - If the operation fails.
 */
export async function hideMenu(): Promise<void> {
  return await desktopCommands.hideMenu();
}

/**
 * Executes a shell command and waits for completion.
 *
 * @example
 * ```ts
 * const curl = await edgebar.shellExec('curl', 'https://www.google.com');
 * console.log(curl.stdout);
 * ```
 *
 * @param {string} command - Path to program executable, or program name
 * (if in $PATH).
 * @param {string | string[]} args - Arguments to pass to the program.
 * @param {Object} options - Spawn options (optional).
 * @throws - If shell permissions are missing.
 */
export async function shellExec<
  TOutput extends string | Uint8Array = string,
>(
  program: string,
  args?: string | string[],
  options?: ShellCommandOptions,
): Promise<ShellExecOutput<TOutput>> {
  return await desktopCommands.shellExec(program, args, options);
}

/**
 * Starts a shell command without waiting for completion. Allows for
 * interaction with the spawned process, such as sending input and killing
 * the process.
 *
 * @example
 * ```ts
 * const ping = await edgebar.shellSpawn('ping', '127.0.0.1 -n 10 -w 3000');
 * ping.onStdout(output => console.log('stdout', output));
 * ping.onStderr(output => console.log('stderr', output));
 * ping.onExit(output => console.log('exit', output));
 *
 * // Interacting with the process.
 * ping.write('Hello, world!');
 * ping.kill();
 * ```
 *
 * @param {string} command - Path to program executable, or program name
 * (if in $PATH).
 * @param {string | string[]} args - Arguments to pass to the program.
 * @param {Object} options - Spawn options (optional).
 * @throws - If shell permissions are missing.
 */
export async function shellSpawn<
  TOutput extends string | Uint8Array = string,
>(
  program: string,
  args?: string | string[],
  options?: ShellCommandOptions,
): Promise<ShellProcess<TOutput>> {
  const processId = await desktopCommands.shellSpawn(
    program,
    args,
    options,
  );

  const stdoutCallbacks: ((data: TOutput) => void)[] = [];
  const stderrCallbacks: ((data: TOutput) => void)[] = [];
  const errorCallbacks: ((data: string) => void)[] = [];
  const exitCallbacks: ((data: {
    exitCode: number | null;
    signal: number | null;
  }) => void)[] = [];

  const unlistenEvents = await listen(
    'shell-emit',
    (event: Event<ShellEmission>) => {
      if (event.payload.pid === processId) {
        const shellEvent = event.payload.event;

        switch (shellEvent.type) {
          case 'stdout':
            stdoutCallbacks.forEach(callback =>
              callback(shellEvent.data as TOutput),
            );
            break;
          case 'stderr':
            stderrCallbacks.forEach(callback =>
              callback(shellEvent.data as TOutput),
            );
            break;
          case 'error':
            errorCallbacks.forEach(callback => callback(shellEvent.data));
            break;
          case 'terminated':
            exitCallbacks.forEach(callback => callback(shellEvent.data));
            unlistenEvents();
            break;
        }
      }
    },
  );

  return {
    processId,
    onStdout: callback => stdoutCallbacks.push(callback),
    onStderr: callback => stderrCallbacks.push(callback),
    onExit: callback => exitCallbacks.push(callback),
    kill: () => desktopCommands.shellKill(processId),
    write: data => desktopCommands.shellWrite(processId, data),
  };
}

export interface ShellProcess<
  TOutput extends string | Uint8Array = string,
> {
  processId: number;
  onStdout: (callback: (line: TOutput) => void) => void;
  onStderr: (callback: (line: TOutput) => void) => void;
  onExit: (
    callback: (status: {
      exitCode: number | null;
      signal: number | null;
    }) => void,
  ) => void;
  kill: () => void;
  write: (data: string | Uint8Array) => void;
}
