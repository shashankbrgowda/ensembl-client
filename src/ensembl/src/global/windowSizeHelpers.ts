import reduce from 'lodash/reduce';

import windowService from 'src/services/window-service';

export const getCurrentMediaSize = (queries: { [key: string]: string }) => {
  return reduce(
    queries,
    (result: string | null, mediaQuery, mediaSize) => {
      const mediaQueryList = getMediaQueryList(mediaQuery);
      if (mediaQueryList.matches) {
        return mediaSize;
      } else {
        return result;
      }
    },
    null
  );
};

export const observeMediaQueries = (
  queries: { [key: string]: string },
  callback: (key: string) => void
) => {
  // First, get instant media query match
  const currentMediaSize = getCurrentMediaSize(queries);
  if (currentMediaSize) {
    callback(currentMediaSize);
  }

  // Second, subscribe to subsequent media query changes
  const observableQueries = Object.entries(queries).map(([key, query]) => {
    const mediaQueryList = getMediaQueryList(query);
    const onChange = (event: MediaQueryListEvent) => {
      if (event.matches) {
        callback(key);
      }
    };
    mediaQueryList.addListener(onChange);
    return { mediaQueryList, onChange };
  });

  const unsubscribe = () => {
    observableQueries.forEach(({ mediaQueryList, onChange }) => {
      mediaQueryList.removeListener(onChange);
    });
  };
  return { unsubscribe };
};

const getMediaQueryList = (query: string) => {
  const matchMedia = windowService.getMatchMedia();
  return matchMedia(query);
};