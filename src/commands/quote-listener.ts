import type { EventCallback, UnlistenFn } from '@/adapters';
import {
  getRunEnv,
  RUN_ENV,
  listenQuotesSyncStartTauri,
  listenQuotesSyncCompleteTauri,
  listenQuotesSyncErrorTauri,
  logger,
} from '@/adapters';

// listenQuotesSyncStart
export const listenQuotesSyncStart = async <T>(handler: EventCallback<T>): Promise<UnlistenFn> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return listenQuotesSyncStartTauri<T>(handler);
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error('Error listen PORTFOLIO_UPDATE_START.');
    throw error;
  }
};

// listenQuotesSyncComplete
export const listenQuotesSyncComplete = async <T>(
  handler: EventCallback<T>,
): Promise<UnlistenFn> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return listenQuotesSyncCompleteTauri<T>(handler);
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error('Error listen PORTFOLIO_UPDATE_COMPLETE.');
    throw error;
  }
};

// listenQuotesSyncError
export const listenQuotesSyncError = async <T>(handler: EventCallback<T>): Promise<UnlistenFn> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return listenQuotesSyncErrorTauri<T>(handler);
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error('Error listen PORTFOLIO_UPDATE_ERROR.');
    throw error;
  }
};
