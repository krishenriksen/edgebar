import { createAudioProvider } from './audio/create-audio-provider';
import type {
  AudioProviderConfig,
  AudioProvider,
} from './audio/audio-provider-types';

import { createSystrayProvider } from './systray/create-systray-provider';
import type {
  SystrayProviderConfig,
  SystrayProvider,
} from './systray/systray-provider-types';

import { createWindowProvider } from './window/create-window-provider';
import type {
  WindowProviderConfig,
  WindowProvider,
} from './window/window-provider-types';

export interface ProviderConfigMap {
  audio: AudioProviderConfig;
  systray: SystrayProviderConfig;
  window: WindowProviderConfig;
}

export interface ProviderMap {
  audio: AudioProvider;
  media: MediaProvider;
  systray: SystrayProvider;
  window: WindowProvider;
}

export type ProviderType = keyof ProviderConfigMap;

export type ProviderConfig = ProviderConfigMap[keyof ProviderConfigMap];

/**
 * Creates a provider, which is a collection of functions and variables
 * that can change over time. Alternatively, multiple providers can be
 * created using {@link createProviderGroup}.
 *
 * The provider will continue to output until its `stop` function is
 * called.
 *
 * @throws If the provider config is invalid. Errors are emitted via the
 * `onError` method.
 */
export function createProvider<T extends ProviderConfig>(
  config: T,
): ProviderMap[T['type']] {
  switch (config.type) {
    case 'audio':
      return createAudioProvider(config) as any;
    case 'systray':
      return createSystrayProvider(config) as any;
    case 'window':
      return createWindowProvider(config) as any;
    default:
      throw new Error('Not a supported provider type.');
  }
}
