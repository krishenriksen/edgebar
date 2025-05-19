import type { Provider } from '../create-base-provider';

export interface WindowProviderConfig {
  type: 'window';
}

export type WindowProvider = Provider<WindowProviderConfig, WindowOutput>;

export interface WindowOutput {
  title: string;
  hwnd: number;
}
