type LogMethod = 'log' | 'warn' | 'error';

const isProductionMode = true; // Set to false to enable debug mode

export function createLogger(section: string) {
  function log(
    consoleLogMethod: LogMethod,
    message: string,
    ...data: unknown[]
  ) {
    if (isProductionMode) return; // Only log if in debug mode

    const date = new Date();
    const timestamp =
      `${date.getHours().toString().padStart(2, '0')}:` +
      `${date.getMinutes().toString().padStart(2, '0')}:` +
      `${date.getSeconds().toString().padStart(2, '0')}:` +
      `${date.getMilliseconds().toString().padStart(3, '0')}`;

    // Clone data to avoid reference changes in Chrome console.
    const clonedData = data.map(tryClone);

    console[consoleLogMethod](
      `%c${timestamp}%c [${section}] %c${message}`,
      'color: #f5f9b4',
      'color: #d0b4f9',
      'color: inherit',
      ...clonedData,
    );
  }

  function debug(message: string, ...data: unknown[]) {
    log('log', message, ...data);
  }

  function info(message: string, ...data: unknown[]) {
    log('log', message, ...data);
  }

  function warn(message: string, ...data: unknown[]) {
    log('warn', message, ...data);
  }

  function error(message: string, ...data: unknown[]) {
    log('error', message, ...data);
  }

  return {
    debug,
    info,
    warn,
    error,
  };
}

function tryClone(data: unknown) {
  if (data === null || data === undefined || data instanceof Error) {
    return data;
  }

  try {
    return structuredClone(data);
  } catch (err) {
    console.warn('Unable to clone data');
    return data;
  }
}
