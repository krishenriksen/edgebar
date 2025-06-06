type LogMethod = 'log' | 'warn' | 'error';

export function createLogger(section?: string) {
  function log(
    consoleLogMethod: LogMethod,
    message: string,
    ...data: unknown[]
  ) {
    const date = new Date();
    const timestamp =
      `${date.getHours().toString().padStart(2, '0')}:` +
      `${date.getMinutes().toString().padStart(2, '0')}:` +
      `${date.getSeconds().toString().padStart(2, '0')}:` +
      `${date.getMilliseconds().toString().padStart(3, '0')}`;

    console[consoleLogMethod](
      `%c[Edgebar] %c${timestamp}%c${section ? ` (${section})` : ''} %c${message}`,
      'color: #4ade80',
      'color: #f5f9b4',
      'color: #d0b4f9',
      'color: inherit',
      ...data,
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