import log, { Logger, LogLevelDesc } from 'loglevel';
import prefix from 'loglevel-plugin-prefix';

const DEFAULT_LOG_LEVEL: LogLevelDesc =
  process.env.NODE_ENV === 'production' ? 'error' : 'info';

prefix.reg(log);
prefix.apply(log, {
  template: '[%t] %l (%n):',
  levelFormatter(level) {
    return level.toUpperCase();
  },
  nameFormatter(name) {
    return name ?? 'Global';
  },
  timestampFormatter: function (date) {
    return date.toTimeString().replace(/.*(\d{2}:\d{2}:\d{2}).*/, '$1');
  },
});

export const getLogger = (
  name: string,
  logLevel: LogLevelDesc = DEFAULT_LOG_LEVEL
): Logger => {
  const logger = log.getLogger(name);
  logger.setLevel(logLevel);
  return logger;
};
